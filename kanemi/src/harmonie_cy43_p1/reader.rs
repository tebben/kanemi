// KNMI uses and old version of Grib files (v1), multiple readers can be found for v2 but v1
// is however not that common. I also don't like the bindings for eccodes, we need another dependency
// and KNMI is using custom tables for their parameters which is a bit of a hassle to get working.
//
// We only work with the Harmonie cy43 P1 grib files, so we can optimize the reader for that. This means
// this reader is not a general purpose Grib v1 reader.
// - We know what messages are in the file
// - We do not need to parse GDS since this is always the same for every message
// - We do not need all the PDS fields, only a few
// - We know which parts have a bitmap section
// - pds length is always 28
// - gds length is always 760
// - value count is always 152100
//
// Knowing these things we can easily index and skip to the parts we want. If messages always
// appear in the same order we can even skip indexing, but we will index for now.
//
// scanning mode 64:
// +i direction, -j direction, points in i direction are consecutive
// +i direction means points scan horizontally across longitude (left to right, increasing longitude).
// -j direction means rows are ordered vertically in decreasing latitude (from top to bottom).
//
// Griv v1 specs: watch out for one-based indexing vs zero-based indexing in our code
// https://codes.ecmwf.int/grib/format/grib1/sections/0/
//
// Used parts of grib1_reader for inspiration: https://github.com/christian-boks/grib1_reader/blob/master/src/lib.rs
//
// ToDo:
// - Test if location to index is correct
// - Add if param has bmp
// - Table lookup by name and level
// - Cleanup
// - Add function to read given values all and at specific locations
// - Error handling
// - Tests
// - Benchmarks

use super::grib_info::GRIBInfo;
use bitstream_io::{BigEndian, BitRead, BitReader};
use std::fs::File;
use std::io::Cursor;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug)]
pub struct IndicatorSection {
    pub section_length: u32,
    pub edition_number: u8,
}

#[derive(Debug)]
pub struct ProductDefinitionSection {
    pub table_version_number: u8,          // GRIB table version number
    pub parameter_code: u8,                // Parameter indicator
    pub level_type: u8,                    // Level type indicator
    pub level: u16,                        // Level value
    pub forecast_time: u8,                 // Forecast time (hours)
    pub originating_center: u8,            // Center that created the GRIB file
    pub generating_process: u8,            // Process that generated the data
    pub grid_identification: u8,           // Grid definition number
    pub flag_presence_absence_gds_bms: u8, // Flag presence/absence of GDS and BMS
    pub reference_time: String,            // Reference time (YYYY-MM-DD HH:MM:SS)
    pub time_range_indicator: u8,          // Time range indicator
}

impl ProductDefinitionSection {
    pub fn has_gds(&self) -> bool {
        self.flag_presence_absence_gds_bms & 128 > 0
    }

    pub fn has_bmp(&self) -> bool {
        self.flag_presence_absence_gds_bms & 64 > 0
    }
}

#[derive(Debug)]
pub struct GridIdentificationSection {
    pub pv_location: u8,
    pub data_representation_type: u8,
    pub latitude_south: f32,
    pub longitude_west: f32,
    pub latitude_north: f32,
    pub longitude_east: f32,
    pub number_of_latitude_points: usize,
    pub number_of_longitude_points: usize,
    pub latitude_spacing: f32,
    pub longitude_spacing: f32,
    pub scanning_mode: u8,
    pub value_count: u32,
}

impl Default for GridIdentificationSection {
    fn default() -> Self {
        GridIdentificationSection {
            pv_location: 33,
            data_representation_type: 0,
            latitude_south: 49.000004,
            longitude_west: 0.0,
            latitude_north: 56.002003,
            longitude_east: 11.281,
            number_of_latitude_points: 390,
            number_of_longitude_points: 390,
            latitude_spacing: 0.017999997,
            longitude_spacing: 0.029000001,
            scanning_mode: 64,
            value_count: 152100,
        }
    }
}

#[derive(Debug)]
pub struct BitmapSection {
    pub number_of_unused_bits_at_end_of_section3: u8,
    pub table_reference: u16,
    pub bmp: Vec<u8>,
}

pub struct GribFile<R> {
    reader: R,
    file_size: u64,
    table: GRIBInfo,
    grid: GridIdentificationSection,
    length_indicator: usize,
    length_pds: usize,
    length_gds: usize,
}

fn read_f32_ibm(data: &[u8]) -> f32 {
    let sign = if (data[0] & 0x80) > 0 { -1.0 } else { 1.0 };
    let a = (data[0] & 0x7f) as i32;
    let b = (((data[1] as i32) << 16) + ((data[2] as i32) << 8) + data[3] as i32) as f32;

    sign * 2.0f32.powi(-24) * b * 16.0f32.powi(a - 64)
}

fn read_i16_be(array: &[u8]) -> i16 {
    let mut val = (array[1] as i16) + (((array[0] & 127) as i16) << 8);
    if array[0] & 0x80 > 0 {
        val = -val;
    }
    val
}

fn read_u16_be(array: &[u8]) -> u16 {
    (array[1] as u16) + ((array[0] as u16) << 8)
}

fn read_u24_be(array: &[u8]) -> u32 {
    (array[2] as u32) + ((array[1] as u32) << 8) + ((array[0] as u32) << 16)
}

impl GribFile<File> {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<GribFile<File>> {
        let file = File::open(path)?;
        let file_size = file.metadata()?.len();

        let mut grib_file = GribFile {
            reader: file,
            file_size,
            table: GRIBInfo::new(),
            grid: GridIdentificationSection::default(),
            length_indicator: 8,
            length_pds: 28,
            length_gds: 760,
        };
        grib_file.create_index(0);

        Ok(grib_file)
    }

    fn read_exact_buffer(&mut self, len: usize) -> io::Result<Vec<u8>> {
        let mut buffer = vec![0u8; len];
        self.reader.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    pub fn read_indicator_section(&mut self) -> io::Result<Option<IndicatorSection>> {
        let buffer = self.read_exact_buffer(8)?;
        let marker = &buffer[0..4];
        if &marker != b"GRIB" {
            match std::str::from_utf8(marker) {
                Ok(marker_str) => println!("Marker as string: {}", marker_str),
                Err(_) => println!("Marker is not valid UTF-8"),
            }
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid GRIB file",
            ));
        }
        let section_length = read_u24_be(&buffer[4..]);

        if section_length > self.file_size as u32 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Section length exceeds file size",
            ));
        }

        let edition_number = buffer[7];
        Ok(Some(IndicatorSection {
            section_length,
            edition_number,
        }))
    }

    pub fn read_product_definition_section(
        &mut self,
    ) -> io::Result<Option<ProductDefinitionSection>> {
        let len = self.get_length();
        let buffer = self.read_exact_buffer(len)?;

        let section = ProductDefinitionSection {
            table_version_number: buffer[3],
            originating_center: buffer[4],
            generating_process: buffer[5],
            grid_identification: buffer[6],
            flag_presence_absence_gds_bms: buffer[7],
            parameter_code: buffer[8],
            level_type: buffer[9],
            level: read_u16_be(&buffer[10..]),
            forecast_time: buffer[17],
            reference_time: format!(
                "{:02}-{:02}-{:02} {:02}:{:02}:{:02}",
                buffer[12], buffer[13], buffer[14], buffer[15], buffer[16], 0
            ),
            time_range_indicator: buffer[20],
        };

        Ok(Some(section))
    }

    pub fn read_grid_identification_section(
        &mut self,
    ) -> io::Result<Option<GridIdentificationSection>> {
        let len = self.get_length();
        let buffer = self.read_exact_buffer(len)?;

        let pv_location = buffer[4];
        let data_representation_type = buffer[5];
        let number_of_latitude_points = u16::from_be_bytes([buffer[6], buffer[7]]) as usize;
        let number_of_longitude_points = u16::from_be_bytes([buffer[8], buffer[9]]) as usize;
        let latitude_south =
            (i32::from_be_bytes([0, buffer[10], buffer[11], buffer[12]]) as f32) * 0.001;
        let longitude_west =
            (i32::from_be_bytes([0, buffer[13], buffer[14], buffer[15]]) as f32) * 0.001;
        let latitude_north =
            (i32::from_be_bytes([0, buffer[17], buffer[18], buffer[19]]) as f32) * 0.001;
        let longitude_east =
            (i32::from_be_bytes([0, buffer[20], buffer[21], buffer[22]]) as f32) * 0.001;
        let latitude_spacing =
            (latitude_north - latitude_south) / (number_of_latitude_points as f32 - 1.0);
        let longitude_spacing =
            (longitude_east - longitude_west) / (number_of_longitude_points as f32 - 1.0);
        let scanning_mode = buffer[27];
        let value_count = number_of_latitude_points as u32 * number_of_longitude_points as u32;

        Ok(Some(GridIdentificationSection {
            pv_location,
            data_representation_type,
            latitude_south,
            longitude_west,
            latitude_north,
            longitude_east,
            number_of_latitude_points,
            number_of_longitude_points,
            latitude_spacing,
            longitude_spacing,
            scanning_mode,
            value_count,
        }))
    }

    pub fn read_bds_section(
        &mut self,
        value_count: usize,
        bitmap: &Option<BitmapSection>,
    ) -> io::Result<()> {
        let len = self.get_length();
        let buffer = self.read_exact_buffer(len)?;
        let binary_scale = read_i16_be(&buffer[4..]);
        let ref_value = read_f32_ibm(&buffer[6..]);
        let bit_count = buffer[10];

        let mut r = BitReader::endian(Cursor::new(&buffer[11..]), BigEndian);
        let mut result = vec![];
        let mut iterations = 0;
        let base: f32 = 2.0;
        let factor = base.powf(binary_scale as f32);

        let mut bitmap_reader = None;
        let uses_bitmap = bitmap.is_some();
        if uses_bitmap {
            bitmap_reader = Some(BitReader::endian(
                Cursor::new(&bitmap.as_ref().unwrap().bmp),
                BigEndian,
            ));
        }

        while iterations < value_count {
            if uses_bitmap {
                let present = match bitmap_reader.as_mut().unwrap().read_bit() {
                    Ok(val) => val,
                    Err(err) => {
                        println!("Bitmap reader error {:?}", err);
                        false
                    }
                };

                if !present {
                    result.push(0.0);
                    iterations += 1;
                    continue;
                }
            }

            match r.read::<u32>(bit_count as u32) {
                Ok(x) => {
                    let y = ref_value + (x as f32) * factor;
                    result.push(y);
                }
                Err(_) => {
                    println!("Error reading value");
                    break;
                }
            };

            iterations += 1;
        }

        Ok(())
    }

    fn read_bitmap_section(&mut self) -> BitmapSection {
        let len = self.get_length();
        let buffer = self.read_exact_buffer(len).unwrap();

        BitmapSection {
            number_of_unused_bits_at_end_of_section3: buffer[3],
            table_reference: read_u16_be(&buffer[4..]),
            bmp: buffer[6..].to_vec(),
        }
    }

    pub fn read_bds_section_from(
        &mut self,
        has_bmp: bool,
        index: usize,
        locations: Option<&Vec<(f32, f32)>>,
    ) -> io::Result<()> {
        let bds_index = index + self.length_indicator + self.length_pds + self.length_gds;
        self.reader.seek(SeekFrom::Start(bds_index as u64)).unwrap();

        let bitmap = if has_bmp {
            Some(self.read_bitmap_section())
        } else {
            None
        };

        let len = self.get_length();
        let buffer = self.read_exact_buffer(len)?;
        let binary_scale = read_i16_be(&buffer[4..]);
        let ref_value = read_f32_ibm(&buffer[6..]);
        let bit_count = buffer[10];

        let mut r = BitReader::endian(Cursor::new(&buffer[11..]), BigEndian);
        let mut result = vec![];

        let factor = (2.0f32).powf(binary_scale as f32);

        let mut bitmap_reader = bitmap
            .as_ref()
            .map(|bmp| BitReader::endian(Cursor::new(bmp.bmp.as_slice()), BigEndian));

        // Decide which read strategy to use
        match locations {
            Some(locs) => self.read_selected_locations(
                &mut r,
                &mut bitmap_reader,
                &mut result,
                locs,
                has_bmp,
                bit_count,
                ref_value,
                factor,
            ),
            None => self.read_all_values(
                &mut r,
                &mut bitmap_reader,
                &mut result,
                has_bmp,
                bit_count,
                ref_value,
                factor,
            ),
        }?;

        println!("{:?}", result);

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn read_selected_locations(
        &self,
        r: &mut BitReader<Cursor<&[u8]>, BigEndian>,
        bitmap_reader: &mut Option<BitReader<Cursor<&[u8]>, BigEndian>>,
        result: &mut Vec<f32>,
        locations: &Vec<(f32, f32)>,
        has_bmp: bool,
        bit_count: u8,
        ref_value: f32,
        factor: f32,
    ) -> io::Result<()> {
        for (lon, lat) in locations {
            let value_index = self.closest_lon_lat_idx(lon, lat);
            let skip_bits = value_index * (bit_count as usize);

            if has_bmp && !self.is_value_present(bitmap_reader, skip_bits)? {
                result.push(0.0);
                continue;
            }

            self.read_and_push_value(r, result, skip_bits, bit_count, ref_value, factor)?;
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn read_all_values(
        &self,
        r: &mut BitReader<Cursor<&[u8]>, BigEndian>,
        bitmap_reader: &mut Option<BitReader<Cursor<&[u8]>, BigEndian>>,
        result: &mut Vec<f32>,
        has_bmp: bool,
        bit_count: u8,
        ref_value: f32,
        factor: f32,
    ) -> io::Result<()> {
        let value_count = self.grid.value_count as usize;
        for _ in 0..value_count {
            if has_bmp && !self.is_value_present(bitmap_reader, 0)? {
                result.push(0.0);
                continue;
            }

            self.read_and_push_value(r, result, 0, bit_count, ref_value, factor)?;
        }
        Ok(())
    }

    fn is_value_present(
        &self,
        bitmap_reader: &mut Option<BitReader<Cursor<&[u8]>, BigEndian>>,
        skip_bits: usize,
    ) -> io::Result<bool> {
        if let Some(reader) = bitmap_reader {
            reader.skip(skip_bits as u32)?;
            return reader.read_bit();
        }
        Ok(true)
    }

    fn read_and_push_value(
        &self,
        r: &mut BitReader<Cursor<&[u8]>, BigEndian>,
        result: &mut Vec<f32>,
        skip_bits: usize,
        bit_count: u8,
        ref_value: f32,
        factor: f32,
    ) -> io::Result<()> {
        r.skip(skip_bits as u32)?;
        match r.read::<u32>(bit_count as u32) {
            Ok(x) => result.push(ref_value + (x as f32) * factor),
            Err(_) => {
                println!("Error reading value");
            }
        }
        Ok(())
    }

    // we find indexes of all messages in the file and store them in the table
    // after reading the indicator and product section we are not interested in this anymore
    // after this we can skip easily to a specific message.
    // If someone wants to read all messages this doesn't slow it down
    // since we are not reading the sections multiple times and can skip around in our reader
    // using the indexes.
    fn create_index(&mut self, start_index: u64) {
        let mut stack = vec![start_index];

        while let Some(index) = stack.pop() {
            self.reader.seek(SeekFrom::Start(index)).unwrap();
            let mut next = 0;

            if let Some(indicator) = self.read_indicator_section().unwrap() {
                next = index + indicator.section_length as u64;

                if let Some(pds) = self.read_product_definition_section().unwrap() {
                    if let Some(info) = self.table.get_parameter_info(
                        pds.parameter_code,
                        pds.level_type,
                        pds.level,
                        pds.time_range_indicator,
                    ) {
                        info.set_byte_index(index as usize);
                    }
                }
            }

            if next < self.file_size {
                stack.push(next); // Push the next index onto the stack
            }
        }
    }

    /// Find the index of the closest longitude and latitude point in the grid
    /// to the given longitude and latitude
    pub fn closest_lon_lat_idx(&self, lon: &f32, lat: &f32) -> usize {
        // Compute indices
        let lon_idx = (((lon - self.grid.longitude_west) / self.grid.longitude_spacing)
            .round()
            .clamp(0.0, (self.grid.number_of_longitude_points - 1) as f32))
            as usize;
        let lat_idx = (((lat - self.grid.latitude_south) / self.grid.latitude_spacing)
            .round()
            .clamp(0.0, (self.grid.number_of_latitude_points - 1) as f32))
            as usize;

        // Return computed 1D index based on scanning mode
        lat_idx * self.grid.number_of_longitude_points as usize + lon_idx
    }

    pub fn read_grib_file_2(&mut self) {
        // Test read temp at level 0 and 50
        let locations = vec![(5.351926, 51.716_8)];

        let pi_tmp_0 = self.table.get_parameter_info(11, 105, 0, 0).unwrap();
        let b_index_0 = pi_tmp_0.byte_index;
        self.read_bds_section_from(false, b_index_0, Some(&locations))
            .unwrap();

        let pi_tmp_50 = self.table.get_parameter_info(11, 105, 50, 0).unwrap();
        let b_index_50 = pi_tmp_50.byte_index;
        self.read_bds_section_from(false, b_index_50, Some(&locations))
            .unwrap();
    }

    pub fn read_grib_file(&mut self, start: u64) -> io::Result<()> {
        self.reader.seek(SeekFrom::Start(start)).unwrap();

        let mut next = 0;

        if let Some(indicator) = self.read_indicator_section()? {
            //println!("{:#?}", indicator);
            next = start + indicator.section_length as u64;

            if let Some(pds) = self.read_product_definition_section()? {
                //println!("{:#?}", pds);
                let table_info = self.table.get_parameter_info(
                    pds.parameter_code,
                    pds.level_type,
                    pds.level,
                    pds.time_range_indicator,
                );
                let _ = match table_info {
                    Some(info) => info.short_name.clone(),
                    None => "Unknown".to_string(),
                };

                let gds = self.read_grid_identification_section()?;
                if let Some(grid) = gds {
                    println!("{:#?}", grid);
                    let mut bitmap: Option<BitmapSection> = None;
                    if pds.has_bmp() {
                        bitmap = Some(self.read_bitmap_section());
                    }

                    let value_count = grid.number_of_latitude_points as usize
                        * grid.number_of_longitude_points as usize;
                    self.read_bds_section(value_count, &bitmap)?;
                }
            }
        }

        if next < self.file_size {
            self.read_grib_file(next)?;
        }

        Ok(())
    }

    pub fn get_length(&mut self) -> usize {
        let mut buffer = [0u8; 3];
        self.reader.read_exact(&mut buffer).unwrap();
        let len = read_u24_be(&buffer[..]) as usize;
        self.reader.seek(SeekFrom::Current(-3)).unwrap();

        len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_grib_file() {
        // time function

        let start = std::time::Instant::now();
        let grib_file = GribFile::open("../example_data/HA43_N20_202412221800_00000_GB");
        grib_file.unwrap().read_grib_file_2();
        let duration = start.elapsed();
        println!("Time elapsed in read_grib_file_2() is: {:?}", duration);
        //grib_file.unwrap().read_grib_file(0);
        assert_eq!(1, 1);
    }
}
