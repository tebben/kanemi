use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ParameterCode(u8);

impl ParameterCode {
    pub fn new(code: u8) -> Self {
        ParameterCode(code)
    }

    // Getter method to retrieve the inner value
    pub fn value(&self) -> u8 {
        self.0
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
pub struct GribMetadata {
    pub code: ParameterCode,
    pub short_name: String,
    pub description: String,
    pub units: String,
    pub level_type: LevelType,
    pub level: u16,
    pub time_range_indicator: TimeRangeIndicator,
    pub has_bmp: bool,
    pub byte_index: Option<usize>,
}

impl GribMetadata {
    pub fn set_byte_index(&mut self, index: usize) {
        self.byte_index = Some(index);
    }
}

#[derive(Debug, Clone)]
pub struct GRIBInfo {
    lookup: HashMap<(u8, u8, u16, u8), GribMetadata>,
    name_lookup: HashMap<(String, u16), (u8, u8, u16, u8)>,
    pub forecast_time: Option<String>,
}

impl Default for GRIBInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl GRIBInfo {
    pub fn new() -> Self {
        let mut table = HashMap::new();
        let mut name_lookup = HashMap::new();

        let parameters = vec![
            GribMetadata {
                code: ParameterCode::new(1),
                short_name: "pmsl".to_string(),
                description: "Pressure altitude above mean sea level".to_string(),
                units: "Pa".to_string(),
                level_type: LevelType::AltitudeAboveSeaLevel,
                level: 0,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(1),
                short_name: "psrf".to_string(),
                description: "Pressure height above ground".to_string(),
                units: "Pa".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(6),
                short_name: "gp".to_string(),
                description: "Geopotential".to_string(),
                units: "m2 s-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(11),
                short_name: "tmp".to_string(),
                description: "Temperature".to_string(),
                units: "K".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(11),
                short_name: "tmp".to_string(),
                description: "Temperature".to_string(),
                units: "K".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 2,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(11),
                short_name: "tmp".to_string(),
                description: "Temperature".to_string(),
                units: "K".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 50,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(11),
                short_name: "tmp".to_string(),
                description: "Temperature".to_string(),
                units: "K".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 100,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(11),
                short_name: "tmp".to_string(),
                description: "Temperature".to_string(),
                units: "K".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 200,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(11),
                short_name: "tmp".to_string(),
                description: "Temperature".to_string(),
                units: "K".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 300,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(11),
                short_name: "isba".to_string(),
                description: "Temperature of nature tile".to_string(),
                units: "K".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 800,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(11),
                short_name: "isba".to_string(),
                description: "Temperature of nature tile".to_string(),
                units: "K".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 801,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(11),
                short_name: "isba".to_string(),
                description: "Temperature of nature tile".to_string(),
                units: "K".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 802,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: true,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(17),
                short_name: "dpt".to_string(),
                description: "Dew-point temperature".to_string(),
                units: "K".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 2,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(20),
                short_name: "vis".to_string(),
                description: "Visibility".to_string(),
                units: "m".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(33),
                short_name: "ugrd".to_string(),
                description: "u-component of wind".to_string(),
                units: "m s-1".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 10,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(33),
                short_name: "ugrd".to_string(),
                description: "u-component of wind".to_string(),
                units: "m s-1".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 50,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(33),
                short_name: "ugrd".to_string(),
                description: "u-component of wind".to_string(),
                units: "m s-1".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 100,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(33),
                short_name: "ugrd".to_string(),
                description: "u-component of wind".to_string(),
                units: "m s-1".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 200,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(33),
                short_name: "ugrd".to_string(),
                description: "u-component of wind".to_string(),
                units: "m s-1".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 300,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(34),
                short_name: "vgrd".to_string(),
                description: "v-component of wind".to_string(),
                units: "m s-1".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 10,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(34),
                short_name: "vgrd".to_string(),
                description: "v-component of wind".to_string(),
                units: "m s-1".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 50,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(34),
                short_name: "vgrd".to_string(),
                description: "v-component of wind".to_string(),
                units: "m s-1".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 100,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(34),
                short_name: "vgrd".to_string(),
                description: "v-component of wind".to_string(),
                units: "m s-1".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 200,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(34),
                short_name: "vgrd".to_string(),
                description: "v-component of wind".to_string(),
                units: "m s-1".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 300,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(52),
                short_name: "rh".to_string(),
                description: "Relative humidity".to_string(),
                units: "%".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 2,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(61),
                short_name: "apcp".to_string(),
                description: "Total precipitation".to_string(),
                units: "kg m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::AccumulatedOverForecastPeriod,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(65),
                short_name: "weasd".to_string(),
                description: "Water equivalent of accumulated snow depth".to_string(),
                units: "kg m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(66),
                short_name: "sd".to_string(),
                description: "Snow depth".to_string(),
                units: "m".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: true,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(67),
                short_name: "mixht".to_string(),
                description: "Mixed layer depth".to_string(),
                units: "m".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(71),
                short_name: "tcdc".to_string(),
                description: "Total cloud cover".to_string(),
                units: "%".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(73),
                short_name: "lcdc".to_string(),
                description: "Low cloud cover".to_string(),
                units: "%".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(74),
                short_name: "mcdc".to_string(),
                description: "Medium cloud cover".to_string(),
                units: "%".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(75),
                short_name: "hcdc".to_string(),
                description: "High cloud cover".to_string(),
                units: "%".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(81),
                short_name: "land".to_string(),
                description: "Landcover".to_string(),
                units: "Proportion".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(111),
                short_name: "nswrs".to_string(),
                description: "Net short-wave radiation flux (surface)".to_string(),
                units: "W m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::AccumulatedOverForecastPeriod,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(112),
                short_name: "nlwrs".to_string(),
                description: "Net long-wave radiation flux (surface)".to_string(),
                units: "W m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::AccumulatedOverForecastPeriod,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(117),
                short_name: "grad".to_string(),
                description: "Global radiation flux".to_string(),
                units: "W m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::AccumulatedOverForecastPeriod,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(122),
                short_name: "shtfl".to_string(),
                description: "Sensible heat flux".to_string(),
                units: "W m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::AccumulatedOverForecastPeriod,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(132),
                short_name: "lhtfl".to_string(),
                description: "Latent heat flux through evaporation".to_string(),
                units: "W m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::AccumulatedOverForecastPeriod,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(162),
                short_name: "csulf".to_string(),
                description: "U-momentum of gusts out of the model".to_string(),
                units: "m s-1".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 10,
                time_range_indicator: TimeRangeIndicator::AccumulatedOverPeriodPart,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(163),
                short_name: "csdlf".to_string(),
                description: "V-momentum of gusts out of the model".to_string(),
                units: "m s-1".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 10,
                time_range_indicator: TimeRangeIndicator::AccumulatedOverPeriodPart,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(181),
                short_name: "lpsxc".to_string(),
                description: "Cumulative sum rain".to_string(),
                units: "kg m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::AccumulatedOverForecastPeriod,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(181),
                short_name: "lpsx".to_string(),
                description: "Rain".to_string(),
                units: "kg m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(184),
                short_name: "hgtyc".to_string(),
                description: "Cumulative sum snow".to_string(),
                units: "kg m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::AccumulatedOverForecastPeriod,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(184),
                short_name: "hgty".to_string(),
                description: "Snow".to_string(),
                units: "kg m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(186),
                short_name: "icng".to_string(),
                description: "Cloud base".to_string(),
                units: "m".to_string(),
                level_type: LevelType::EntireAtmosphere,
                level: 0,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: true,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(201),
                short_name: "icwatc".to_string(),
                description: "Cumulative sum graupel".to_string(),
                units: "kg m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::AccumulatedOverForecastPeriod,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(201),
                short_name: "icwat".to_string(),
                description: "Graupel".to_string(),
                units: "kg m-2".to_string(),
                level_type: LevelType::HeightAboveGround,
                level: 0,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
            GribMetadata {
                code: ParameterCode::new(201),
                short_name: "icwat".to_string(),
                description: "Column integrated graupel".to_string(),
                units: "kg m-2".to_string(),
                level_type: LevelType::EntireAtmosphere,
                level: 0,
                time_range_indicator: TimeRangeIndicator::Instant,
                has_bmp: false,
                byte_index: None,
            },
        ];

        // Insert each parameter into the table
        for param in parameters {
            table.insert(
                (
                    param.code.0,
                    param.level_type as u8,
                    param.level,
                    param.time_range_indicator as u8,
                ),
                param.clone(),
            );

            name_lookup.insert(
                (param.short_name.clone(), param.level),
                (
                    param.code.0,
                    param.level_type as u8,
                    param.level,
                    param.time_range_indicator as u8,
                ),
            );
        }

        GRIBInfo {
            lookup: table,
            name_lookup,
            forecast_time: None,
        }
    }

    /// Set the byte index for a parameter
    pub fn set_byte_index(
        &mut self,
        code: u8,
        level_type: u8,
        level: u16,
        time_range_indicator: u8,
        index: usize,
    ) {
        let k = &(code, level_type, level, time_range_indicator);
        if let Some(param) = self.lookup.get_mut(k) {
            param.set_byte_index(index);
        }
    }

    /// Get all parameters
    pub fn get_all_parameters(&self) -> Vec<&GribMetadata> {
        self.lookup.values().collect()
    }

    /// get a copy of all parameters
    pub fn get_all_parameters_copy(&self) -> Vec<GribMetadata> {
        self.lookup.values().cloned().collect()
    }

    /// Get a parameter by code, level type, level, and time range indicator
    pub fn get_parameter(
        &self,
        code: u8,
        level_type: u8,
        level: u16,
        time_range_indicator: u8,
    ) -> Option<&GribMetadata> {
        let k = &(code, level_type, level, time_range_indicator);
        self.lookup.get(k)
    }

    /// Get a parameter by name and level
    pub fn get_parameter_by_name(&self, name: &str, level: u16) -> Option<&GribMetadata> {
        let &(code, level_type, level, time_range_indicator) = self
            .name_lookup
            .get(&(name.to_lowercase().to_string(), level))?;
        self.get_parameter(code, level_type, level, time_range_indicator)
    }

    pub fn get_parameters_by_name(
        &self,
        parameters: Option<&Vec<(&str, u16)>>,
    ) -> Vec<&GribMetadata> {
        if let Some(params) = parameters {
            params
                .iter()
                .map(|(name, level)| self.get_parameter_by_name(name, *level).unwrap())
                .collect()
        } else {
            self.get_all_parameters()
        }
    }
}
