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
// Knowing these things we can easily index and skip to the parts we want.
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
// msg order:
// 1: (11,0), 2: (6,0), 3: (65,0), 4: (132,0), 5: (122,0), 6: (117,0), 7: (33,50), 8: (34,50), 9: (33,100),
// 10: (34,100), 11: (33,200), 12: (34,200), 13: (33,300), 14: (34,300), 15: (11,50), 16: (11,100), 17: (11,200),
// 18: (11,300), 19: (111,0), 20: (112,0), 21: (181,0), 22: (184,0), 23: (201,0), 24: (11,2), 25: (52,2), 26: (33,10),
// 27: (34,10), 28: (162,10), 29: (163,10), 30: (75,0), 31: (74,0), 32: (73,0), 33: (71,0), 34: (67,0), 35: (181,0),
// 36: (184,0), 37: (201,0), 38: (1,0), 39: (1,0), 40: (81,0), 41: (11,802), 42: (66,0), 43: (61,0), 44: (20,0),
// 45: (17,2), 46: (186,0), 47: (201,0), 48: (11,800), 49: (11,801)

use super::bits::{read_f32_ibm, read_i16_be, read_u16_be, read_u24_be};
use super::errors::GribError;
use super::grib_info::{GRIBInfo, GribMetadata};
use bitstream_io::{BigEndian, BitRead, BitReader};
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Cursor;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;
use std::vec;

#[derive(Debug, Serialize)]
pub struct GribResponse {
    pub time: String,
    pub locations: Option<Vec<(f32, f32)>>,
    pub results: Vec<GribResult>,
}

#[derive(Debug, Serialize)]
pub struct GribResult {
    pub name: String,
    pub level: u16,
    pub values: Vec<f32>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct IndicatorSection {
    pub section_length: u32,
    pub edition_number: u8,
}

#[derive(Debug)]
#[allow(dead_code)]
struct ProductDefinitionSection {
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

#[allow(dead_code)]
impl ProductDefinitionSection {
    pub fn has_gds(&self) -> bool {
        self.flag_presence_absence_gds_bms & 128 > 0
    }

    pub fn has_bmp(&self) -> bool {
        self.flag_presence_absence_gds_bms & 64 > 0
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct GridIdentificationSection {
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

#[derive(Clone)]
#[allow(dead_code)]
struct Location {
    pub lon: f32,
    pub lat: f32,
    pub index: usize,
}

impl Location {
    fn new(lon: f32, lat: f32, index: usize) -> Location {
        Location { lon, lat, index }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct BitmapSection {
    pub number_of_unused_bits_at_end_of_section3: u8,
    pub table_reference: u16,
    pub bmp: Vec<u8>,
}

#[derive(Debug)]
pub struct CY43P1Reader {
    file: RefCell<File>,
    file_size: u64,
    metadata: RefCell<GRIBInfo>,
    grid: GridIdentificationSection,
    length_indicator: usize,
    length_pds: usize,
    length_gds: usize,
}

impl CY43P1Reader {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<CY43P1Reader, GribError> {
        if fs::metadata(path.as_ref()).is_err() {
            return Err(GribError::FileNotFound(
                "No such file or directory".to_string(),
            ));
        }

        let file = File::open(path).map_err(|e| GribError::ReadError(e.to_string()))?;
        let metadata = file
            .metadata()
            .map_err(|e| GribError::ReadError(e.to_string()))?;

        let file_size = metadata.len();
        let grib_file = CY43P1Reader {
            file: RefCell::new(file),
            file_size,
            metadata: RefCell::new(GRIBInfo::new()),
            grid: GridIdentificationSection::default(),
            length_indicator: 8,
            length_pds: 28,
            length_gds: 760,
        };

        Ok(grib_file)
    }

    /// Get the data for the given parameters and locations
    /// If no parameters are given, all available parameters are returned
    /// If no locations are given, all values are returned
    ///
    /// # Example
    /// ```
    /// use kanemi::harmonie_cy43_p1::reader::CY43P1Reader;
    ///
    /// let parameters = vec![("tmp".to_string(), 0), ("isba".to_string(), 802)];
    /// let locations = vec![(5.351926, 51.716801), (4.9130824, 52.34228)];
    ///
    /// let reader = CY43P1Reader::open("../example_data/HA43_N20_202412221800_00000_GB").unwrap();
    /// let response = reader.get(Some(parameters), Some(locations));
    /// ```
    pub fn get(
        &self,
        parameters: Option<Vec<(String, u16)>>,
        locations: Option<Vec<(f32, f32)>>,
    ) -> Result<GribResponse, GribError> {
        let mut file = self.file.borrow_mut();
        let table_clone = self.metadata.borrow().clone();
        let parameter_info = table_clone
            .get_parameters_by_name(parameters.as_ref())?
            .clone();

        self.create_index(&mut file, &parameter_info);

        let indexed_locations = self.create_locations(locations.as_ref())?;

        // refresh parameter_info with updated byte indexes
        let table_clone = self.metadata.borrow().clone();
        let parameter_info = table_clone
            .get_parameters_by_name(parameters.as_ref())?
            .clone();
        let time = self
            .metadata
            .borrow()
            .forecast_time
            .clone()
            .unwrap_or_default();
        let mut grib_response = GribResponse {
            time,
            locations,
            results: vec![],
        };

        //iterate over all parameters and read bds
        for param in parameter_info {
            let has_bmp = param.has_bmp;
            let byte_index = param.byte_index.unwrap();
            let values =
                self.read_bds_section(&mut file, has_bmp, byte_index, indexed_locations.as_ref())?;

            let grib_result = GribResult {
                name: param.short_name.clone(),
                level: param.level,
                values,
            };

            grib_response.results.push(grib_result);
        }

        Ok(grib_response)
    }

    /// Get all available parameters in the GRIB file
    ///
    /// # Example
    /// ```
    /// use kanemi::harmonie_cy43_p1::reader::CY43P1Reader;
    ///
    /// let reader = CY43P1Reader::open("../example_data/HA43_N20_202412221800_00000_GB").unwrap();
    /// let parameters = reader.available_parameters();
    /// ```
    pub fn available_parameters(&self) -> Vec<GribMetadata> {
        self.metadata.borrow().get_all_parameters_copy()
    }

    /// Find the index of the closest longitude and latitude point in the grid
    /// to the given longitude and latitude
    ///
    /// # Example
    /// ```
    /// use kanemi::harmonie_cy43_p1::reader::CY43P1Reader;
    ///
    /// let reader = CY43P1Reader::open("../example_data/HA43_N20_202412221800_00000_GB").unwrap();
    /// let idx = reader.closest_lon_lat_idx(5.351926, 51.716801).unwrap();
    /// ```
    pub fn closest_lon_lat_idx(&self, lon: f32, lat: f32) -> Result<usize, GribError> {
        if lon < self.grid.longitude_west || lon > self.grid.longitude_east {
            return Err(GribError::OutOfBounds(
                "Longitude out of bounds".to_string(),
            ));
        }
        if lat < self.grid.latitude_south || lat > self.grid.latitude_north {
            return Err(GribError::OutOfBounds("Latitude out of bounds".to_string()));
        }

        // Compute indices
        let lon_idx =
            ((lon - self.grid.longitude_west) / self.grid.longitude_spacing).round() as usize;
        let lat_idx =
            ((lat - self.grid.latitude_south) / self.grid.latitude_spacing).round() as usize;

        // Return computed 1D index based on scanning mode
        Ok(lat_idx * self.grid.number_of_longitude_points + lon_idx)
    }

    fn read_indicator_section(
        &self,
        file: &mut File,
    ) -> Result<Option<IndicatorSection>, GribError> {
        let buffer = self.read_exact_buffer(file, 8)?;
        let marker = &buffer[0..4];
        if &marker != b"GRIB" {
            return Err(GribError::InvalidFile("Unable to read marker".to_string()));
        }
        let section_length = read_u24_be(&buffer[4..]);
        if section_length > self.file_size as u32 {
            return Err(GribError::InvalidLength(
                "Invalid section length".to_string(),
            ));
        }

        let edition_number = buffer[7];
        Ok(Some(IndicatorSection {
            section_length,
            edition_number,
        }))
    }

    fn read_product_definition_section(
        &self,
        file: &mut File,
    ) -> Result<Option<ProductDefinitionSection>, GribError> {
        let len = self.get_message_length(file)?;
        let buffer = self.read_exact_buffer(file, len)?;
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
                "20{:02}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
                buffer[12], buffer[13], buffer[14], buffer[15], buffer[16], 0
            ),
            time_range_indicator: buffer[20],
        };

        Ok(Some(section))
    }

    #[allow(dead_code)]
    fn read_grid_identification_section(
        &self,
        file: &mut File,
    ) -> Result<Option<GridIdentificationSection>, GribError> {
        let len = self.get_message_length(file)?;
        let buffer = self.read_exact_buffer(file, len)?;
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

    fn read_bitmap_section(&self, file: &mut File) -> Result<BitmapSection, GribError> {
        let len = self.get_message_length(file)?;
        let buffer = self.read_exact_buffer(file, len).unwrap();

        Ok(BitmapSection {
            number_of_unused_bits_at_end_of_section3: buffer[3],
            table_reference: read_u16_be(&buffer[4..]),
            bmp: buffer[6..].to_vec(),
        })
    }

    fn read_bds_section(
        &self,
        file: &mut File,
        has_bmp: bool,
        index: usize,
        locations: Option<&Vec<Location>>,
    ) -> Result<Vec<f32>, GribError> {
        let bds_index = index + self.length_indicator + self.length_pds + self.length_gds;
        file.seek(SeekFrom::Start(bds_index as u64)).unwrap();

        let bitmap = if has_bmp {
            Some(self.read_bitmap_section(file)?)
        } else {
            None
        };

        let len = self.get_message_length(file)?;
        let buffer = self.read_exact_buffer(file, len).unwrap();
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
        let _ = match locations {
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
        };

        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    fn read_selected_locations(
        &self,
        r: &mut BitReader<Cursor<&[u8]>, BigEndian>,
        bitmap_reader: &mut Option<BitReader<Cursor<&[u8]>, BigEndian>>,
        result: &mut Vec<f32>,
        locations: &Vec<Location>,
        has_bmp: bool,
        bit_count: u8,
        ref_value: f32,
        factor: f32,
    ) -> io::Result<()> {
        for loc in locations {
            let mut skip_bits = loc.index as u32 * bit_count as u32;

            if has_bmp {
                let present =
                    self.is_value_present_with_bds_index(bitmap_reader, loc.index as u32)?;
                match present {
                    (true, Some(bds_index)) => {
                        skip_bits = bds_index as u32 * bit_count as u32;
                    }
                    (false, _) => {
                        result.push(9999.0);
                        continue;
                    }
                    _ => {
                        println!("Error reading value");
                    }
                }
            }

            self.read_and_push_value(r, result, skip_bits, bit_count, ref_value, factor, true)?;
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
            if has_bmp {
                let present = bitmap_reader.as_mut().unwrap().read_bit().unwrap_or(false);
                if !present {
                    result.push(9999.0);
                    continue;
                }
            }

            self.read_and_push_value(r, result, 0, bit_count, ref_value, factor, false)?;
        }
        Ok(())
    }

    // bmp contains if a value is present or not, since the index of a value changes when data is missing
    // we need to find the correct index of a value in the BDS section
    fn is_value_present_with_bds_index(
        &self,
        bitmap_reader: &mut Option<BitReader<Cursor<&[u8]>, BigEndian>>,
        skip_bits: u32,
    ) -> io::Result<(bool, Option<usize>)> {
        if let Some(reader) = bitmap_reader {
            reader.seek_bits(SeekFrom::Start(0))?;

            let mut bds_index = 0;
            for _ in 0..skip_bits {
                if reader.read_bit()? {
                    bds_index += 1;
                }
            }

            let value_present = reader.read_bit()?;
            if !value_present {
                return Ok((false, None));
            }

            Ok((true, Some(bds_index)))
        } else {
            Ok((true, Some(skip_bits as usize)))
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn read_and_push_value(
        &self,
        r: &mut BitReader<Cursor<&[u8]>, BigEndian>,
        result: &mut Vec<f32>,
        skip_bits: u32,
        bit_count: u8,
        ref_value: f32,
        factor: f32,
        seek: bool,
    ) -> io::Result<()> {
        if seek {
            r.seek_bits(SeekFrom::Start(0)).unwrap();
            r.skip(skip_bits)?;
        }

        match r.read::<u32>(bit_count as u32) {
            Ok(x) => {
                let value = ref_value + (x as f32) * factor;
                result.push(value);
            }
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
    fn create_index(&self, file: &mut File, parameters: &Vec<&GribMetadata>) {
        let mut stack = vec![0];
        let target_params: HashSet<_> = parameters
            .iter()
            .map(|param| {
                (
                    param.code.value(),
                    param.level,
                    param.level_type as u8,
                    param.time_range_indicator as u8,
                )
            })
            .collect();

        // if all parameters already have a byte index we don't need to index the file
        if parameters.iter().all(|p| p.byte_index.is_some()) {
            return;
        }

        let tartget_parameter_count = parameters.len();
        let mut found_parameters = 0;

        while let Some(index) = stack.pop() {
            file.seek(SeekFrom::Start(index)).unwrap();

            let mut next = 0;

            if let Some(indicator) = self.read_indicator_section(file).unwrap() {
                next = index + indicator.section_length as u64;

                if let Some(pds) = self.read_product_definition_section(file).unwrap() {
                    if target_params.contains(&(
                        pds.parameter_code,
                        pds.level,
                        pds.level_type,
                        pds.time_range_indicator,
                    )) {
                        found_parameters += 1;

                        let mut metadata = self.metadata.borrow_mut();
                        if metadata.forecast_time.is_none() {
                            metadata.forecast_time = Some(pds.reference_time.clone());
                        }

                        metadata.set_byte_index(
                            pds.parameter_code,
                            pds.level_type,
                            pds.level,
                            pds.time_range_indicator,
                            index as usize,
                        );

                        if found_parameters == tartget_parameter_count {
                            return;
                        }
                    }
                }
            }

            if next < self.file_size {
                stack.push(next); // Push the next index onto the stack
            }
        }
    }

    // create locations from &Vec<(f64, f64)>
    fn create_locations(
        &self,
        locations: Option<&Vec<(f32, f32)>>,
    ) -> Result<Option<Vec<Location>>, GribError> {
        locations.map_or(Ok(None), |locs| {
            locs.iter()
                .map(|(lon, lat)| {
                    self.closest_lon_lat_idx(*lon, *lat)
                        .map(|index| Location::new(*lon, *lat, index))
                })
                .collect::<Result<Vec<_>, _>>()
                .map(Some)
        })
    }

    /// Read a buffer of a given length from the file
    fn read_exact_buffer(&self, file: &mut File, len: usize) -> Result<Vec<u8>, GribError> {
        let mut buffer = vec![0u8; len];
        file.read_exact(&mut buffer)
            .map_err(|e| GribError::ReadError(e.to_string()))?;
        Ok(buffer)
    }

    /// Get the length of a message
    fn get_message_length(&self, file: &mut File) -> Result<usize, GribError> {
        let mut buffer = [0u8; 3];
        file.read_exact(&mut buffer)
            .map_err(|e| GribError::MessageLengthError(e.to_string()))?;
        let len = read_u24_be(&buffer[..]) as usize;
        file.seek(SeekFrom::Current(-3)).unwrap();

        Ok(len)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::harmonie_cy43_p1::reader::grib_info::{LevelType, TimeRangeIndicator};

    const FILE_PATH1: &str = "../example_data/HA43_N20_202412221800_00000_GB";

    #[test]
    fn get_parameters_locations() {
        let parameters = vec![("tmp".to_string(), 0), ("isba".to_string(), 802)];
        let locations = vec![(5.351926, 51.716_8), (4.913082420058467, 52.3422859189378)];

        let grib_file = CY43P1Reader::open(FILE_PATH1).unwrap();
        let response = grib_file.get(Some(parameters), Some(locations)).unwrap();

        assert_eq!(response.locations.unwrap().len(), 2);
        assert_eq!(response.results.len(), 2);
        assert_eq!(response.time, "2024-12-22T18:00:00Z");

        // test no bmp
        let tmp_result = response.results.iter().find(|r| r.name == "tmp").unwrap();
        assert_eq!(tmp_result.level, 0);
        assert_eq!(tmp_result.values.len(), 2);
        assert_eq!(tmp_result.values[0], 276.26367);
        assert_eq!(tmp_result.values[1], 277.34326);

        // test with bmp
        let isba_result = response.results.iter().find(|r| r.name == "isba").unwrap();
        assert_eq!(isba_result.level, 802);
        assert_eq!(isba_result.values.len(), 2);
        assert_eq!(isba_result.values[0], 277.11752);
        assert_eq!(isba_result.values[1], 279.4792);
    }

    #[test]
    fn test_corner_locations_all_params() {
        let locations = vec![
            (0.0, 49.000004),    //idx 0
            (11.281, 49.000004), //idx 389
            (0.0, 56.002003),    //idx 151710
            (11.281, 56.002003), //idx 152099
        ];

        let grib_file = CY43P1Reader::open(FILE_PATH1).unwrap();
        let response = grib_file.get(None, Some(locations)).unwrap();

        assert_eq!(response.locations.unwrap().len(), 4);
        assert_eq!(response.results.len(), 49);

        let tmp_results = response
            .results
            .iter()
            .find(|r| r.name == "tmp" && r.level == 0)
            .unwrap();
        assert_eq!(tmp_results.values.len(), 4);
        assert_eq!(tmp_results.values[0], 279.03223);
        assert_eq!(tmp_results.values[1], 272.89478);
        assert_eq!(tmp_results.values[2], 282.67017);
        assert_eq!(tmp_results.values[3], 278.8506);

        let isba_result = response
            .results
            .iter()
            .find(|r| r.name == "isba" && r.level == 802)
            .unwrap();
        assert_eq!(isba_result.values.len(), 4);
        assert_eq!(isba_result.values[0], 279.97);
        assert_eq!(isba_result.values[1], 273.80685);
        assert_eq!(isba_result.values[2], 9999.0);
        assert_eq!(isba_result.values[3], 278.47852);
    }

    #[test]
    fn test_load_grib_file_no_locations() {
        let parameters = vec![("tmp".to_string(), 0), ("isba".to_string(), 802)];

        let grib_file = CY43P1Reader::open(FILE_PATH1).unwrap();
        let response = grib_file.get(Some(parameters), None).unwrap();

        assert!(response.locations.is_none());
        assert_eq!(response.results.len(), 2);

        let tmp_result = response.results.iter().find(|r| r.name == "tmp").unwrap();
        assert_eq!(tmp_result.level, 0);
        assert_eq!(tmp_result.values.len(), 152100);
        assert_eq!(tmp_result.values[152099], 278.8506);

        let isba_result = response.results.iter().find(|r| r.name == "isba").unwrap();
        assert_eq!(isba_result.level, 802);
        assert_eq!(isba_result.values.len(), 152100);
        assert_eq!(isba_result.values[152099], 278.47852);
    }

    #[test]
    fn test_available_parameters() {
        let grib_file = CY43P1Reader::open(FILE_PATH1).unwrap();
        let parameters = grib_file.available_parameters();

        assert_eq!(parameters.len(), 49);

        let tmp = parameters
            .iter()
            .find(|p| p.short_name == "tmp" && p.level == 0)
            .unwrap();
        assert_eq!(tmp.code.value(), 11);
        assert_eq!(tmp.level, 0);
        assert_eq!(tmp.level_type, LevelType::HeightAboveGround);
        assert_eq!(tmp.time_range_indicator, TimeRangeIndicator::Instant);

        let isba = parameters
            .iter()
            .find(|p| p.short_name == "isba" && p.level == 802)
            .unwrap();

        assert_eq!(isba.code.value(), 11);
        assert_eq!(isba.level, 802);
        assert_eq!(isba.level_type, LevelType::HeightAboveGround);
        assert_eq!(isba.time_range_indicator, TimeRangeIndicator::Instant);
        assert_eq!(isba.has_bmp, true);
    }

    #[test]
    fn test_open_file_not_found() {
        let result = CY43P1Reader::open("not_a_file");
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "File not found: No such file or directory"
        );
    }

    #[test]
    fn test_parameter_error() {
        let grib_file = CY43P1Reader::open(FILE_PATH1).unwrap();
        let result = grib_file.get(
            Some(vec![("not_a_param".to_string(), 0), ("tmp".to_string(), 0)]),
            None,
        );

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Parameter not found: name: not_a_param, level: 0"
        );
    }

    #[test]
    fn test_out_of_bounds() {
        let grib_file = CY43P1Reader::open(FILE_PATH1).unwrap();
        let result = grib_file.closest_lon_lat_idx(0.0, 0.0);

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Out of bounds: Latitude out of bounds"
        );

        let result = grib_file.closest_lon_lat_idx(100.0, 0.0);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Out of bounds: Longitude out of bounds"
        );
    }
}
