use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct StationShort {
    pub code: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Station {
    /// Station code
    pub code: String,
    /// Station name
    pub name: String,
    /// No idea
    pub wsi: String,
    /// Latitude position of the station
    pub latitude: f64,
    /// Longitude position of the station
    pub longitude: f64,
    /// Altitude of the station
    pub height: f64,
    /// Rainfall Duration in last Hour, min
    pub d1h: f64,
    /// Wind Direction 10 Min Average with MD, °
    pub dd: f64,
    /// Wind Direction Sensor 10 Min Minimum with MD, °
    pub dn: f64,
    /// Precipitation Duration (Rain Gauge) 10 Min Sum, sec
    pub dr: f64,
    /// Wind Direction 10 Min Std Dev with MD, °
    pub dsd: f64,
    /// Wind Direction 10 Min Std Dev with MD, °
    pub dx: f64,
    /// Wind Speed at 10m 10 Min Average with MD, m/s
    pub ff: f64,
    /// Wind Speed Sensor 10 Min Average with MD, m/s
    pub ffs: f64,
    /// Wind Speed 10 Min Std Dev with MD, m/s
    pub fsd: f64,
    /// Wind Gust at 10m Maximum last 10 Min Interval, m/s
    pub fx: f64,
    /// Wind Gust Sensor Maximum last 10 Min Interval, m/s
    pub fxs: f64,
    /// Wind Gust at 10m 10 Min Maximum with MD, m/s
    pub gff: f64,
    /// Wind Gust Sensor 10 Min Maximum with MD, m/s
    pub gffs: f64,
    /// Cloud Base, ft
    pub h: f64,
    /// Cloud Base First Layer, ft
    pub h1: f64,
    /// Cloud Base Second Layer, ft
    pub h2: f64,
    /// Cloud Base Third Layer, ft
    pub h3: f64,
    /// Cloud Base Ceilometer Algorithm, ft
    pub hc: f64,
    /// Cloud Base First Layer Ceilometer, ft
    pub hc1: f64,
    /// Cloud Base Second Layer Ceilometer, ft
    pub hc2: f64,
    /// Cloud Base Third Layer Ceilometer, ft
    pub hc3: f64,
    /// Total cloud cover, octa
    pub n: f64,
    /// Cloud Amount First Layer, octa
    pub n1: f64,
    /// Cloud Amount Second Layer, octa
    pub n2: f64,
    /// Cloud Amount Third Layer, octa
    pub n3: f64,
    /// Total Cloud Cover Ceilometer, octa
    pub nc: f64,
    /// Cloud Amount First Layer Ceilometer, octa
    pub nc1: f64,
    /// Cloud Amount Second Layer Ceilometer, octa
    pub nc2: f64,
    /// Cloud Amount Third Layer Ceilometer, octa
    pub nc3: f64,
    /// Air Pressure at Station Level 1 Min Average, hPa
    pub p0: f64,
    /// Air Pressure at Sea Level 1 Min Average, hPa
    pub pp: f64,
    /// Precipitation Intensity (PWS) 10 Min Average, mm/h
    pub pg: f64,
    /// Precipitation Duration (PWS) 10 Min Sum, sec
    pub pr: f64,
    /// Air Pressure at Sensor Level 1 Min Average, hPa
    pub ps: f64,
    /// Corrected Precipitation Type 10 Min Maximum, code KNMI Handboek waarnemingen
    pub pwc: f64,
    /// Global Radiation 1 Hour Sum, J/cm²
    pub q1h: f64,
    /// Global Radiation 24 Hour Sum, J/cm²
    pub q24h: f64,
    /// Global Solar Radiation 10 Min Average, W/m²
    pub qg: f64,
    /// Global Solar Radiation 10 Min Minimum, W/m²
    pub qgn: f64,
    /// Global Solar Radiation 10 Min Maximum, W/m²
    pub qgx: f64,
    /// QNH 1 Min Average, hPa
    pub qnh: f64,
    /// Rainfall in last 12 Hours, mm
    pub r12h: f64,
    /// Rainfall in last Hour, mm
    pub r1h: f64,
    /// Rainfall in last 24 Hours, mm
    pub r24h: f64,
    /// Rainfall in last 6 Hours, mm
    pub r6h: f64,
    /// Precipitation Intensity (Rain Gauge) 10 Min Average, mm/h
    pub rg: f64,
    /// Relative Humidity 1.5m 1 Min Average, %
    pub rh: f64,
    /// Relative Humidity 10 Min Average, %
    pub rh10: f64,
    /// Wind Speed Average last 1 Hour, m/s
    pub sav1h: f64,
    /// Wind Speed Maximum last 1 Hour, m/s
    pub sax1h: f64,
    /// Wind Speed Maximum last 3 Hours, m/s
    pub sax3h: f64,
    /// Wind Speed Maximum last 6 Hours, m/s
    pub sax6h: f64,
    /// Squall Indicator, code WMO table 4680
    pub sq: f64,
    /// Sunshine Duration, min
    pub ss: f64,
    /// Wind Gust Maximum last 1 Hour, m/s
    pub sx1h: f64,
    /// Wind Gust Maximum last 3 Hours, m/s
    pub sx3h: f64,
    /// Wind Gust Maximum last 6 Hours, m/s
    pub sx6h: f64,
    /// Ambient Temperature 10 Min Average, °C
    pub t10: f64,
    /// Ambient Temperature 1.5m 10 Min Average, °C
    pub ta: f64,
    /// Wet Bulb Temperature 1.5m 10 Min Average, °C
    pub tb: f64,
    /// Soil Temperature -5cm 10 Min Average, °C
    pub tb1: f64,
    /// Soil Temperature -5cm Minimum last 6 Hours, °C
    pub tb1n6: f64,
    /// Soil Temperature -5cm Maximum last 6 Hours, °C
    pub tb1x6: f64,
    /// Soil Temperature -10cm 10 Min Average, °C
    pub tb2: f64,
    /// Soil Temperature -10cm Minimum last 6 Hours, °C
    pub tb2n6: f64,
    /// Soil Temperature -10cm Maximum last 6 Hours, °C
    pub tb2x6: f64,
    /// Soil Temperature -20cm 10 Min Average, °C
    pub tb3: f64,
    /// Soil Temperature -50cm 10 Min Average, °C
    pub tb4: f64,
    /// Soil Temperature -100cm 10 Min Average, °C
    pub tb5: f64,
    /// Dew Point Temperature 1.5m 1 Min Average, °C
    pub td: f64,
    /// Dew Point Temperature 10 Min Average, °C
    pub td10: f64,
    /// Grass Temperature 10cm 10 Min Average, °C
    pub tg: f64,
    /// Grass Temperature 10cm 10 Min Minimum, °C
    pub tgn: f64,
    /// Grass Temperature Minimum last 12 Hours, °C
    pub tgn12: f64,
    /// Grass Temperature Minimum last 14 Hours, °C
    pub tgn14: f64,
    /// Grass Temperature Minimum last 6 Hours, °C
    pub tgn6: f64,
    /// Ambient Temperature 1.5m 10 Min Minimum, °C
    pub tn: f64,
    /// Air Temperature Minimum last 12 Hours, °C
    pub tn12: f64,
    /// Air Temperature Minimum last 14 Hours, °C
    pub tn14: f64,
    /// Air Temperature Minimum last 6 Hours, °C
    pub tn6: f64,
    /// SIAM Ambient Temperature 10 Min Average Std Dev, °C
    pub tsd: f64,
    /// Ambient Temperature 1.5m 10 Min Maximum, °C
    pub tx: f64,
    /// Air Temperature Maximum last 12 Hours, °C
    pub tx12: f64,
    /// Air Temperature Maximum last 24 Hours, °C
    pub tx24: f64,
    /// Air Temperature Maximum last 6 Hours, °C
    pub tx6: f64,
    /// Horizontal Visibility 10 Min Average, m
    pub vv: f64,
    /// Past Weather Indicator, code WMO table 4680
    pub w10: f64,
    /// Past Weather Indicator for Previous 10 Min Interval, code WMO table 4680
    pub w10_10: f64,
    /// wawa Weather Code, code WMO table 4680
    pub ww: f64,
    /// Background Luminance 10 Min Average, cd/m²
    pub ww_10: f64,
    /// Meteorological Optical Range 10 Min Average, m
    pub zm: f64,
}
