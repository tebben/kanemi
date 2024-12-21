use crate::actual_synoptic_observations::models::{Station, StationShort};
use crate::errors::DatasetError;
use crate::utils::haversine_distance;
use chrono::{DateTime, Duration, Utc};
use netcdf::File;

#[derive(Debug)]
pub struct Dataset {
    pub filepath: String,
    pub file: File,
    pub datetime: Option<DateTime<Utc>>,
    pub stations: Option<Vec<StationShort>>,
}

impl Dataset {
    pub fn new(filepath: String) -> Result<Dataset, DatasetError> {
        if filepath.is_empty() || !std::path::Path::new(&filepath).exists() {
            return Err(DatasetError::FileNotFound(format!(
                "File not found: {}",
                filepath
            )));
        }

        let netcdf_file = netcdf::open(filepath.clone()).unwrap();
        let mut ds = Dataset {
            filepath,
            file: netcdf_file,
            datetime: None,
            stations: None,
        };
        ds.load_stations_short();
        ds.load_time();

        Ok(ds)
    }

    /// Load all stations but only a few attributes
    fn load_stations_short(&mut self) {
        let station_code = self.load_string_variable_values("station");
        let station_name = self.load_string_variable_values("stationname");
        let lat = self.load_f64_variable_values("lat");
        let lon = self.load_f64_variable_values("lon");

        let mut stations = Vec::new();
        for i in 0..station_name.len() {
            stations.push(StationShort {
                code: station_code[i].clone(),
                name: station_name[i].clone(),
                latitude: lat[i],
                longitude: lon[i],
            });
        }

        self.stations = Some(stations);
    }

    fn load_time(&mut self) {
        let time = self.load_f64_variable_values("time");
        let time = time[0] as i64;
        let time = DateTime::from_timestamp(time, 0);
        // Subtract 20 years (20 years * 365.25 days/year * 86400 seconds/day)
        let adjusted_time = time.unwrap() - Duration::days((20.0 * 365.25) as i64);

        // Assign the adjusted DateTime
        self.datetime = Some(adjusted_time);
    }

    pub fn get_closest_station(&self, longitude: f64, latitude: f64) -> (Station, u64) {
        let stations = self.stations.as_ref().unwrap();
        let mut closest_station_idx = 0;
        let mut min_distance = f64::MAX;

        for (i, station) in stations.iter().enumerate() {
            let distance =
                haversine_distance(station.longitude, station.latitude, longitude, latitude);
            if distance < min_distance {
                min_distance = distance;
                closest_station_idx = i;
            }
        }

        let min_distance = min_distance.round() as u64;

        (self.load_station(closest_station_idx), min_distance)
    }

    pub fn get_stations(&self) -> Vec<Station> {
        let wsi = self.load_string_variable_values("wsi");
        let station_code = self.load_string_variable_values("station");
        let station_name = self.load_string_variable_values("stationname");
        let lat = self.load_f64_variable_values("lat");
        let lon = self.load_f64_variable_values("lon");
        let height = self.load_f64_variable_values("height");
        let d1h = self.load_f64_variable_values("D1H");
        let dd = self.load_f64_variable_values("dd");
        let dn = self.load_f64_variable_values("dn");
        let dr = self.load_f64_variable_values("dr");
        let dsd = self.load_f64_variable_values("dsd");
        let dx = self.load_f64_variable_values("dx");
        let ff = self.load_f64_variable_values("ff");
        let ffs = self.load_f64_variable_values("ffs");
        let fsd = self.load_f64_variable_values("fsd");
        let fx = self.load_f64_variable_values("fx");
        let fxs = self.load_f64_variable_values("fxs");
        let gff = self.load_f64_variable_values("gff");
        let gffs = self.load_f64_variable_values("gffs");
        let h = self.load_f64_variable_values("h");
        let h1 = self.load_f64_variable_values("h1");
        let h2 = self.load_f64_variable_values("h2");
        let h3 = self.load_f64_variable_values("h3");
        let hc = self.load_f64_variable_values("hc");
        let hc1 = self.load_f64_variable_values("hc1");
        let hc2 = self.load_f64_variable_values("hc2");
        let hc3 = self.load_f64_variable_values("hc3");
        let n = self.load_f64_variable_values("n");
        let n1 = self.load_f64_variable_values("n1");
        let n2 = self.load_f64_variable_values("n2");
        let n3 = self.load_f64_variable_values("n3");
        let nc = self.load_f64_variable_values("nc");
        let nc1 = self.load_f64_variable_values("nc1");
        let nc2 = self.load_f64_variable_values("nc2");
        let nc3 = self.load_f64_variable_values("nc3");
        let p0 = self.load_f64_variable_values("p0");
        let pp = self.load_f64_variable_values("pp");
        let pg = self.load_f64_variable_values("pg");
        let pr = self.load_f64_variable_values("pr");
        let ps = self.load_f64_variable_values("ps");
        let pwc = self.load_f64_variable_values("pwc");
        let q1h = self.load_f64_variable_values("Q1H");
        let q24h = self.load_f64_variable_values("Q24H");
        let qg = self.load_f64_variable_values("qg");
        let qgn = self.load_f64_variable_values("qgn");
        let qgx = self.load_f64_variable_values("qgx");
        let qnh = self.load_f64_variable_values("qnh");
        let r12h = self.load_f64_variable_values("R12H");
        let r1h = self.load_f64_variable_values("R1H");
        let r24h = self.load_f64_variable_values("R24H");
        let r6h = self.load_f64_variable_values("R6H");
        let rg = self.load_f64_variable_values("rg");
        let rh = self.load_f64_variable_values("rh");
        let rh10 = self.load_f64_variable_values("rh10");
        let sav1h = self.load_f64_variable_values("Sav1H");
        let sax1h = self.load_f64_variable_values("Sax1H");
        let sax3h = self.load_f64_variable_values("Sax3H");
        let sax6h = self.load_f64_variable_values("Sax6H");
        let sq = self.load_f64_variable_values("sq");
        let ss = self.load_f64_variable_values("ss");
        let sx1h = self.load_f64_variable_values("Sx1H");
        let sx3h = self.load_f64_variable_values("Sx3H");
        let sx6h = self.load_f64_variable_values("Sx6H");
        let t10 = self.load_f64_variable_values("t10");
        let ta = self.load_f64_variable_values("ta");
        let tb = self.load_f64_variable_values("tb");
        let tb1 = self.load_f64_variable_values("tb1");
        let tb1n6 = self.load_f64_variable_values("Tb1n6");
        let tb1x6 = self.load_f64_variable_values("Tb1x6");
        let tb2 = self.load_f64_variable_values("tb2");
        let tb2n6 = self.load_f64_variable_values("Tb2n6");
        let tb2x6 = self.load_f64_variable_values("Tb2x6");
        let tb3 = self.load_f64_variable_values("tb3");
        let tb4 = self.load_f64_variable_values("tb4");
        let tb5 = self.load_f64_variable_values("tb5");
        let td = self.load_f64_variable_values("td");
        let td10 = self.load_f64_variable_values("td10");
        let tg = self.load_f64_variable_values("tg");
        let tgn = self.load_f64_variable_values("tgn");
        let tgn12 = self.load_f64_variable_values("Tgn12");
        let tgn14 = self.load_f64_variable_values("Tgn14");
        let tgn6 = self.load_f64_variable_values("Tgn6");
        let tn = self.load_f64_variable_values("tn");
        let tn12 = self.load_f64_variable_values("Tn12");
        let tn14 = self.load_f64_variable_values("Tn14");
        let tn6 = self.load_f64_variable_values("Tn6");
        let tsd = self.load_f64_variable_values("tsd");
        let tx = self.load_f64_variable_values("tx");
        let tx12 = self.load_f64_variable_values("Tx12");
        let tx24 = self.load_f64_variable_values("Tx24");
        let tx6 = self.load_f64_variable_values("Tx6");
        let vv = self.load_f64_variable_values("vv");
        let w10 = self.load_f64_variable_values("W10");
        let w10_10 = self.load_f64_variable_values("W10-10");
        let ww = self.load_f64_variable_values("ww");
        let ww_10 = self.load_f64_variable_values("ww-10");
        let zm = self.load_f64_variable_values("zm");

        let mut stations = Vec::new();
        for i in 0..station_name.len() {
            stations.push(Station {
                wsi: wsi[i].clone(),
                code: station_code[i].clone(),
                name: station_name[i].clone(),
                latitude: lat[i],
                longitude: lon[i],
                height: height[i],
                d1h: d1h[i],
                dd: dd[i],
                dn: dn[i],
                dr: dr[i],
                dsd: dsd[i],
                dx: dx[i],
                ff: ff[i],
                ffs: ffs[i],
                fsd: fsd[i],
                fx: fx[i],
                fxs: fxs[i],
                gff: gff[i],
                gffs: gffs[i],
                h: h[i],
                h1: h1[i],
                h2: h2[i],
                h3: h3[i],
                hc: hc[i],
                hc1: hc1[i],
                hc2: hc2[i],
                hc3: hc3[i],
                n: n[i],
                n1: n1[i],
                n2: n2[i],
                n3: n3[i],
                nc: nc[i],
                nc1: nc1[i],
                nc2: nc2[i],
                nc3: nc3[i],
                p0: p0[i],
                pp: pp[i],
                pg: pg[i],
                pr: pr[i],
                ps: ps[i],
                pwc: pwc[i],
                q1h: q1h[i],
                q24h: q24h[i],
                qg: qg[i],
                qgn: qgn[i],
                qgx: qgx[i],
                qnh: qnh[i],
                r12h: r12h[i],
                r1h: r1h[i],
                r24h: r24h[i],
                r6h: r6h[i],
                rg: rg[i],
                rh: rh[i],
                rh10: rh10[i],
                sav1h: sav1h[i],
                sax1h: sax1h[i],
                sax3h: sax3h[i],
                sax6h: sax6h[i],
                sq: sq[i],
                ss: ss[i],
                sx1h: sx1h[i],
                sx3h: sx3h[i],
                sx6h: sx6h[i],
                t10: t10[i],
                ta: ta[i],
                tb: tb[i],
                tb1: tb1[i],
                tb1n6: tb1n6[i],
                tb1x6: tb1x6[i],
                tb2: tb2[i],
                tb2n6: tb2n6[i],
                tb2x6: tb2x6[i],
                tb3: tb3[i],
                tb4: tb4[i],
                tb5: tb5[i],
                td: td[i],
                td10: td10[i],
                tg: tg[i],
                tgn: tgn[i],
                tgn12: tgn12[i],
                tgn14: tgn14[i],
                tgn6: tgn6[i],
                tn: tn[i],
                tn12: tn12[i],
                tn14: tn14[i],
                tn6: tn6[i],
                tsd: tsd[i],
                tx: tx[i],
                tx12: tx12[i],
                tx24: tx24[i],
                tx6: tx6[i],
                vv: vv[i],
                w10: w10[i],
                w10_10: w10_10[i],
                ww: ww[i],
                ww_10: ww_10[i],
                zm: zm[i],
            });
        }

        stations
    }

    fn load_station(&self, index: usize) -> Station {
        Station {
            wsi: self.load_string_variable_value(index, "wsi"),
            code: self.load_string_variable_value(index, "station"),
            name: self.load_string_variable_value(index, "stationname"),
            latitude: self.load_f64_variable_value(index, "lat"),
            longitude: self.load_f64_variable_value(index, "lon"),
            height: self.load_f64_variable_value(index, "height"),
            d1h: self.load_f64_variable_value(index, "D1H"),
            dd: self.load_f64_variable_value(index, "dd"),
            dn: self.load_f64_variable_value(index, "dn"),
            dr: self.load_f64_variable_value(index, "dr"),
            dsd: self.load_f64_variable_value(index, "dsd"),
            dx: self.load_f64_variable_value(index, "dx"),
            ff: self.load_f64_variable_value(index, "ff"),
            ffs: self.load_f64_variable_value(index, "ffs"),
            fsd: self.load_f64_variable_value(index, "fsd"),
            fx: self.load_f64_variable_value(index, "fx"),
            fxs: self.load_f64_variable_value(index, "fxs"),
            gff: self.load_f64_variable_value(index, "gff"),
            gffs: self.load_f64_variable_value(index, "gffs"),
            h: self.load_f64_variable_value(index, "h"),
            h1: self.load_f64_variable_value(index, "h1"),
            h2: self.load_f64_variable_value(index, "h2"),
            h3: self.load_f64_variable_value(index, "h3"),
            hc: self.load_f64_variable_value(index, "hc"),
            hc1: self.load_f64_variable_value(index, "hc1"),
            hc2: self.load_f64_variable_value(index, "hc2"),
            hc3: self.load_f64_variable_value(index, "hc3"),
            n: self.load_f64_variable_value(index, "n"),
            n1: self.load_f64_variable_value(index, "n1"),
            n2: self.load_f64_variable_value(index, "n2"),
            n3: self.load_f64_variable_value(index, "n3"),
            nc: self.load_f64_variable_value(index, "nc"),
            nc1: self.load_f64_variable_value(index, "nc1"),
            nc2: self.load_f64_variable_value(index, "nc2"),
            nc3: self.load_f64_variable_value(index, "nc3"),
            p0: self.load_f64_variable_value(index, "p0"),
            pp: self.load_f64_variable_value(index, "pp"),
            pg: self.load_f64_variable_value(index, "pg"),
            pr: self.load_f64_variable_value(index, "pr"),
            ps: self.load_f64_variable_value(index, "ps"),
            pwc: self.load_f64_variable_value(index, "pwc"),
            q1h: self.load_f64_variable_value(index, "Q1H"),
            q24h: self.load_f64_variable_value(index, "Q24H"),
            qg: self.load_f64_variable_value(index, "qg"),
            qgn: self.load_f64_variable_value(index, "qgn"),
            qgx: self.load_f64_variable_value(index, "qgx"),
            qnh: self.load_f64_variable_value(index, "qnh"),
            r12h: self.load_f64_variable_value(index, "R12H"),
            r1h: self.load_f64_variable_value(index, "R1H"),
            r24h: self.load_f64_variable_value(index, "R24H"),
            r6h: self.load_f64_variable_value(index, "R6H"),
            rg: self.load_f64_variable_value(index, "rg"),
            rh: self.load_f64_variable_value(index, "rh"),
            rh10: self.load_f64_variable_value(index, "rh10"),
            sav1h: self.load_f64_variable_value(index, "Sav1H"),
            sax1h: self.load_f64_variable_value(index, "Sax1H"),
            sax3h: self.load_f64_variable_value(index, "Sax3H"),
            sax6h: self.load_f64_variable_value(index, "Sax6H"),
            sq: self.load_f64_variable_value(index, "sq"),
            ss: self.load_f64_variable_value(index, "ss"),
            sx1h: self.load_f64_variable_value(index, "Sx1H"),
            sx3h: self.load_f64_variable_value(index, "Sx3H"),
            sx6h: self.load_f64_variable_value(index, "Sx6H"),
            t10: self.load_f64_variable_value(index, "t10"),
            ta: self.load_f64_variable_value(index, "ta"),
            tb: self.load_f64_variable_value(index, "tb"),
            tb1: self.load_f64_variable_value(index, "tb1"),
            tb1n6: self.load_f64_variable_value(index, "Tb1n6"),
            tb1x6: self.load_f64_variable_value(index, "Tb1x6"),
            tb2: self.load_f64_variable_value(index, "tb2"),
            tb2n6: self.load_f64_variable_value(index, "Tb2n6"),
            tb2x6: self.load_f64_variable_value(index, "Tb2x6"),
            tb3: self.load_f64_variable_value(index, "tb3"),
            tb4: self.load_f64_variable_value(index, "tb4"),
            tb5: self.load_f64_variable_value(index, "tb5"),
            td: self.load_f64_variable_value(index, "td"),
            td10: self.load_f64_variable_value(index, "td10"),
            tg: self.load_f64_variable_value(index, "tg"),
            tgn: self.load_f64_variable_value(index, "tgn"),
            tgn12: self.load_f64_variable_value(index, "Tgn12"),
            tgn14: self.load_f64_variable_value(index, "Tgn14"),
            tgn6: self.load_f64_variable_value(index, "Tgn6"),
            tn: self.load_f64_variable_value(index, "tn"),
            tn12: self.load_f64_variable_value(index, "Tn12"),
            tn14: self.load_f64_variable_value(index, "Tn14"),
            tn6: self.load_f64_variable_value(index, "Tn6"),
            tsd: self.load_f64_variable_value(index, "tsd"),
            tx: self.load_f64_variable_value(index, "tx"),
            tx12: self.load_f64_variable_value(index, "Tx12"),
            tx24: self.load_f64_variable_value(index, "Tx24"),
            tx6: self.load_f64_variable_value(index, "Tx6"),
            vv: self.load_f64_variable_value(index, "vv"),
            w10: self.load_f64_variable_value(index, "W10"),
            w10_10: self.load_f64_variable_value(index, "W10-10"),
            ww: self.load_f64_variable_value(index, "ww"),
            ww_10: self.load_f64_variable_value(index, "ww-10"),
            zm: self.load_f64_variable_value(index, "zm"),
        }
    }

    pub fn load_f64_variable_values(&self, var_name: &str) -> Vec<f64> {
        let variable = self.file.variable(var_name).unwrap();
        variable.get_values::<f64, _>(..).unwrap()
    }

    pub fn load_f64_variable_value(&self, index: usize, var_name: &str) -> f64 {
        let variable = self.file.variable(var_name).unwrap();
        let dimensions = variable.dimensions();
        let indices = match dimensions.len() {
            1 => vec![index],
            2 => vec![index, 0],
            _ => vec![index],
        };
        variable.get_value::<f64, _>(indices).unwrap()
    }

    pub fn load_string_variable_values(&self, var_name: &str) -> Vec<String> {
        let variable = self.file.variable(var_name).unwrap();
        (0..variable.len())
            .map(|i| variable.get_string(i).unwrap())
            .collect()
    }

    pub fn load_string_variable_value(&self, index: usize, var_name: &str) -> String {
        let variable = self.file.variable(var_name).unwrap();
        variable.get_string(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Timelike};

    use crate::{actual_synoptic_observations::dataset::Dataset, errors::DatasetError};

    #[test]
    fn test_file_not_found() {
        assert!(matches!(
            Dataset::new("".to_string()).unwrap_err(),
            DatasetError::FileNotFound(_)
        ));

        assert!(matches!(
            Dataset::new("./doesnotexist.1".to_string()).unwrap_err(),
            DatasetError::FileNotFound(_)
        ));
    }

    #[test]
    fn test_closest() {
        let dataset =
            Dataset::new("../example_data/KMDS__OPER_P___10M_OBS_L2_202412201930.nc".to_string())
                .unwrap();

        let (closest_station, distance) = dataset.get_closest_station(5.32144283, 51.68726598);
        assert_eq!(closest_station.name, "Herwijnen");
        assert_eq!(closest_station.code, "06356");
        assert_eq!(distance, 22505);

        let (closest_station, distance) =
            dataset.get_closest_station(4.873631390167688, 52.35557797273756);
        assert_eq!(closest_station.name, "Schiphol Airport");
        assert_eq!(closest_station.code, "06240");
        assert_eq!(distance, 7122);
    }

    #[test]
    fn test_time() {
        let dataset =
            Dataset::new("../example_data/KMDS__OPER_P___10M_OBS_L2_202412201930.nc".to_string())
                .unwrap();

        let datetime = dataset.datetime.unwrap();
        assert_eq!(datetime.year(), 2024);
        assert_eq!(datetime.month(), 12);
        assert_eq!(datetime.day(), 20);
        assert_eq!(datetime.hour(), 19);
        assert_eq!(datetime.minute(), 30);
    }

    #[test]
    fn test_all_stations() {
        let dataset =
            Dataset::new("../example_data/KMDS__OPER_P___10M_OBS_L2_202412201930.nc".to_string())
                .unwrap();

        let stations = dataset.get_stations();
        assert_eq!(stations.len(), 58);
    }
}
