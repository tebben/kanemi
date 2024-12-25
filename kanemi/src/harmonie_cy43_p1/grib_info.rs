use std::collections::HashMap;

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
