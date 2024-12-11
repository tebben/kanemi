#[derive(Clone)]
pub struct ReverseOptions {
    /// only return the best match, example: true
    pub best_match: Option<bool>,
    /// location to reverse geocode using lon,lat, example: 52.374,4.900
    pub lonlat: Option<(f64, f64)>,
    /// distance to search around location in meters, example: 1000
    pub distance: Option<u32>,
    /// location to reverse geocode using rd x,y, example: 122000,487000
    pub rd: Option<(f64, f64)>,
    /// which types to return
    pub types: Option<Vec<String>>,
    /// fields to return, example: bron,weergavenaam
    pub fl: Option<Vec<String>>,
    /// filter query, example: bron:BAG, example 2: type:(gemeente OR woonplaats OR weg OR postcode OR adres)
    pub fq: Option<String>,
    /// start index, default: 0
    pub start: Option<i32>,
    /// number of rows to return, default: 10, max: 100
    pub rows: Option<i32>,
}

impl Default for ReverseOptions {
    fn default() -> Self {
        Self {
            best_match: Some(false),
            lonlat: None,
            distance: None,
            rd: None,
            types: Some(vec!["adres".to_string()]),
            fl: Some(vec![
                "id".to_string(),
                "type".to_string(),
                "weergavenaam".to_string(),
                "score".to_string(),
                "afstand".to_string(),
            ]),
            fq: None,
            start: Some(0),
            rows: Some(10),
        }
    }
}

impl ReverseOptions {
    pub fn construct_query(&self) -> String {
        let mut options = self.clone();

        // if best match is true, overwrite params in options: rows to 1, sort on score desc, start on 0
        if let Some(true) = self.best_match {
            options.rows = Some(1);
            options.start = Some(0);
        }

        let mut query = String::new();

        if let Some((x, y)) = self.rd {
            query.push_str(&format!("&X={}", x));
            query.push_str(&format!("&Y={}", y));
        } else if let Some((longitude, latitude)) = self.lonlat {
            query.push_str(&format!("&lon={}", longitude));
            query.push_str(&format!("&lat={}", latitude));
        }

        if let Some(distance) = self.distance {
            query.push_str(&format!("&distance={}", distance));
        }

        if let Some(types) = &self.types {
            for t in types {
                query.push_str(&format!("&type={}", t));
            }
        }

        if let Some(fl) = &self.fl {
            let fl = fl.join(" ");
            query.push_str(&format!("&fl={}", fl));
        }

        if let Some(fq) = &self.fq {
            query.push_str(&format!("&fq={}", fq));
        }

        if let Some(start) = self.start {
            query.push_str(&format!("&start={}", start));
        }

        if let Some(rows) = self.rows {
            query.push_str(&format!("&rows={}", rows));
        }

        query
    }
}
