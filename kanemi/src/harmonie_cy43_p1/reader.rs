// This is a reader specific to the KNMI Harmonie cy43 P1 grib files so we can optimize for that
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
// Used parts of grib1_reader for inspiration: https://github.com/christian-boks/grib1_reader/blob/master/src/lib.rs

use bitstream_io::{BigEndian, BitRead, BitReader};
use std::collections::HashMap;
use std::fs::File;
use std::io::Cursor;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ParameterCode(u8);

impl ParameterCode {
    pub fn new(code: u8) -> Self {
        ParameterCode(code)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum LevelType {
    IsobaricLevel = 100,
    AltitudeAboveSeaLevel = 103,
    HeightAboveGround = 105,
    HybridLevel = 109,
    EntireAtmosphere = 200,
}

impl LevelType {
    pub fn from_u16(value: u8) -> Option<Self> {
        match value {
            100 => Some(LevelType::IsobaricLevel),
            103 => Some(LevelType::AltitudeAboveSeaLevel),
            105 => Some(LevelType::HeightAboveGround),
            109 => Some(LevelType::HybridLevel),
            200 => Some(LevelType::EntireAtmosphere),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TimeRangeIndicator {
    Instant = 0,
    AccumulatedOverPeriodPart = 2,
    AccumulatedOverForecastPeriod = 4,
}

impl TimeRangeIndicator {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(TimeRangeIndicator::Instant),
            2 => Some(TimeRangeIndicator::AccumulatedOverPeriodPart),
            4 => Some(TimeRangeIndicator::AccumulatedOverForecastPeriod),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MessageInfo {
    pub code: ParameterCode,
    pub short_name: String,
    pub description: String,
    pub units: String,
    pub level_type: LevelType,
    pub levels: Vec<u16>,
    pub time_range_indicator: TimeRangeIndicator,
    pub byte_index: usize,
}

impl MessageInfo {
    pub fn set_byte_index(&mut self, index: usize) {
        self.byte_index = index;
    }
}

pub struct GRIBInfo {
    table: HashMap<(u8, u8, u16, u8), MessageInfo>,
}

impl Default for GRIBInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl GRIBInfo {
    pub fn new() -> Self {
        let mut table = HashMap::new();

        let parameters = vec![
            MessageInfo {
                code: ParameterCode::new(1),
                short_name: "PMSL".to_string(),
                description: "Pressure altitude above mean sea level".to_string(),
                units: "Pa".to_string(),
                level_type: LevelType::AltitudeAboveSeaLevel,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(1),
                short_name: "PSRF".to_string(),
                description: "Pressure height above ground".to_string(),
                units: "Pa".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(6),
                short_name: "GP".to_string(),
                description: "Geopotential".to_string(),
                units: "m2 s-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(11),
                short_name: "TMP".to_string(),
                description: "Temperature".to_string(),
                units: "K".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0, 2, 50, 100, 200, 300],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(11),
                short_name: "ISBA".to_string(),
                description: "Temperature of nature tile".to_string(),
                units: "K".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![800, 801, 802],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(17),
                short_name: "DPT".to_string(),
                description: "Dew-point temperature".to_string(),
                units: "K".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![2],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(20),
                short_name: "VIS".to_string(),
                description: "Visibility".to_string(),
                units: "m".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(33),
                short_name: "UGRD".to_string(),
                description: "u-component of wind".to_string(),
                units: "m s-1".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![10, 50, 100, 200, 300],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(34),
                short_name: "VGRD".to_string(),
                description: "v-component of wind".to_string(),
                units: "m s-1".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![10, 50, 100, 200, 300],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(52),
                short_name: "RH".to_string(),
                description: "Relative humidity".to_string(),
                units: "%".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![2],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(61),
                short_name: "APCP".to_string(),
                description: "Total precipitation".to_string(),
                units: "kg m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::AccumulatedOverForecastPeriod,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(65),
                short_name: "WEASD".to_string(),
                description: "Water equivalent of accumulated snow depth".to_string(),
                units: "kg m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(66),
                short_name: "SD".to_string(),
                description: "Snow depth".to_string(),
                units: "m".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(67),
                short_name: "MIXHT".to_string(),
                description: "Mixed layer depth".to_string(),
                units: "m".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(71),
                short_name: "TCDC".to_string(),
                description: "Total cloud cover".to_string(),
                units: "%".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(73),
                short_name: "LCDC".to_string(),
                description: "Low cloud cover".to_string(),
                units: "%".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(74),
                short_name: "MCDC".to_string(),
                description: "Medium cloud cover".to_string(),
                units: "%".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(75),
                short_name: "HCDC".to_string(),
                description: "High cloud cover".to_string(),
                units: "%".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(81),
                short_name: "LAND".to_string(),
                description: "Landcover".to_string(),
                units: "Proportion".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(111),
                short_name: "NSWRS".to_string(),
                description: "Net short-wave radiation flux (surface)".to_string(),
                units: "W m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::AccumulatedOverForecastPeriod,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(112),
                short_name: "NLWRS".to_string(),
                description: "Net long-wave radiation flux (surface)".to_string(),
                units: "W m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::AccumulatedOverForecastPeriod,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(117),
                short_name: "GRAD".to_string(),
                description: "Global radiation flux".to_string(),
                units: "W m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::AccumulatedOverForecastPeriod,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(122),
                short_name: "SHTFL".to_string(),
                description: "Sensible heat flux".to_string(),
                units: "W m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::AccumulatedOverForecastPeriod,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(132),
                short_name: "LHTFL".to_string(),
                description: "Latent heat flux through evaporation".to_string(),
                units: "W m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::AccumulatedOverForecastPeriod,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(162),
                short_name: "CSULF".to_string(),
                description: "U-momentum of gusts out of the model".to_string(),
                units: "m s-1".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![10],
                time_range_indicator: TimeRangeIndicator::AccumulatedOverPeriodPart,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(163),
                short_name: "CSDLF".to_string(),
                description: "V-momentum of gusts out of the model".to_string(),
                units: "m s-1".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![10],
                time_range_indicator: TimeRangeIndicator::AccumulatedOverPeriodPart,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(181),
                short_name: "LPSX".to_string(),
                description: "Cumulative sum rain".to_string(),
                units: "kg m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::AccumulatedOverForecastPeriod,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(181),
                short_name: "LPSX".to_string(),
                description: "Rain".to_string(),
                units: "kg m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(184),
                short_name: "HGTY".to_string(),
                description: "Cumulative sum snow".to_string(),
                units: "kg m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::AccumulatedOverForecastPeriod,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(184),
                short_name: "HGTY".to_string(),
                description: "Snow".to_string(),
                units: "kg m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(186),
                short_name: "ICNG".to_string(),
                description: "Cloud base".to_string(),
                units: "m".to_string(),
                level_type: LevelType::EntireAtmosphere,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(201),
                short_name: "ICWAT".to_string(),
                description: "Cumulative sum graupel".to_string(),
                units: "kg m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::AccumulatedOverForecastPeriod,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(201),
                short_name: "ICWAT".to_string(),
                description: "Graupel".to_string(),
                units: "kg m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
            MessageInfo {
                code: ParameterCode::new(201),
                short_name: "ICWAT".to_string(),
                description: "Column integrated graupel".to_string(),
                units: "kg m-2".to_string(),
                level_type: LevelType::EntireAtmosphere,
                levels: vec![0],
                time_range_indicator: TimeRangeIndicator::Instant,
                byte_index: 0,
            },
        ];

        // Insert each parameter into the table
        for param in parameters {
            for &level in &param.levels {
                table.insert(
                    (
                        param.code.0,
                        param.level_type as u8,
                        level,
                        param.time_range_indicator as u8,
                    ),
                    param.clone(), // Clone to insert into the table
                );
            }
        }

        GRIBInfo { table }
    }

    pub fn get_parameter_info(
        &mut self,
        code: u8,
        level_type: u8,
        level: u16,
        time_range_indicator: u8,
    ) -> Option<&mut MessageInfo> {
        let k = &(code, level_type, level, time_range_indicator);
        self.table.get_mut(k)
    }
}

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
    pub grid_code: u8,
    pub latitude_south: f32,
    pub longitude_west: f32,
    pub latitude_north: f32,
    pub longitude_east: f32,
    pub number_of_latitude_points: u16,
    pub number_of_longitude_points: u16,
    pub latitude_spacing: f32,
    pub longitude_spacing: f32,
}

#[derive(Debug)]
///Bit-map section
pub struct BitmapSection {
    pub number_of_unused_bits_at_end_of_section3: u8,
    pub table_reference: u16,
    pub bmp: Vec<u8>,
}

pub struct GribFile<R> {
    reader: R,
    file_size: u64,
    table: GRIBInfo,
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

// fn read_i24_be(array: &[u8]) -> i32 {
//     let mut val = (array[2] as i32) + ((array[1] as i32) << 8) + (((array[0] & 127) as i32) << 16);
//     if array[0] & 0x80 > 0 {
//         val = -val;
//     }
//     val
// }

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
        Ok(GribFile {
            reader: file,
            file_size,
            table: GRIBInfo::new(),
        })
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

        let grid_code = buffer[5];
        let number_of_latitude_points = u16::from_be_bytes([buffer[6], buffer[7]]);
        let number_of_longitude_points = u16::from_be_bytes([buffer[8], buffer[9]]);
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

        Ok(Some(GridIdentificationSection {
            grid_code,
            latitude_south,
            longitude_west,
            latitude_north,
            longitude_east,
            number_of_latitude_points,
            number_of_longitude_points,
            latitude_spacing,
            longitude_spacing,
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
        value_count: usize,
    ) -> io::Result<()> {
        // move to the start of the section
        self.reader.seek(SeekFrom::Start(index as u64)).unwrap();

        let mut bitmap: Option<BitmapSection> = None;
        if has_bmp {
            bitmap = Some(self.read_bitmap_section());
        }

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
        if bitmap.is_some() {
            bitmap_reader = Some(BitReader::endian(
                Cursor::new(&bitmap.as_ref().unwrap().bmp),
                BigEndian,
            ));
        }

        while iterations < value_count {
            if has_bmp {
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

        println!("{:?}", result);

        Ok(())
    }

    // we find indexes of all messages in the file and store them in the table
    // after reading the indicator and product section we are not interested in this anymore
    // after this we can skip easily to a specific message.
    // If someone wants to read all messages this doesn't slow it down
    // since we are not reading the sections multiple times and can skip around in our reader
    // using the indexes.
    pub fn index(&mut self, index: u64) {
        self.reader.seek(SeekFrom::Start(index)).unwrap();
        let mut next = 0;

        if let Some(indicator) = self.read_indicator_section().unwrap() {
            next = index + indicator.section_length as u64;

            if let Some(pds) = self.read_product_definition_section().unwrap() {
                let table_info = self.table.get_parameter_info(
                    pds.parameter_code,
                    pds.level_type,
                    pds.level,
                    pds.time_range_indicator,
                );

                if let Some(info) = table_info {
                    info.set_byte_index(index as usize);
                }
            }
        }

        if next < self.file_size {
            self.index(next);
        }
    }

    pub fn read_grib_file_2(&mut self) {
        self.index(0);

        let pi_tmp = self.table.get_parameter_info(11, 105, 0, 0).unwrap();
        let b_index = pi_tmp.byte_index;
        let bds_index = b_index + 8 + 28 + 760; // start msg + indicator + pds + gds
                                                //let value_count = 152100;
        let value_count = 50;
        self.read_bds_section_from(false, bds_index, value_count)
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
                    //     println!("{:#?}", grid);
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
        let grib_file = GribFile::open("../example_data/HA43_N20_202412221800_00000_GB");
        grib_file.unwrap().read_grib_file_2();
        //grib_file.unwrap().read_grib_file(0);
        assert_eq!(1, 1);
    }
}