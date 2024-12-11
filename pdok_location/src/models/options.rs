#[derive(Clone)]
pub struct FreeOptions {
    /// search query, example: amsterdam
    pub q: String,
    /// only return the best match, example: true
    pub best_match: Option<bool>,
    /// sort results on distance, example: 52.374,4.900
    pub coordinates: Option<(f64, f64)>,
    /// fields to return, example: bron,weergavenaam
    pub fl: Option<String>,
    /// filter query, example: bron:BAG, example 2: type:(gemeente OR woonplaats OR weg OR postcode OR adres)
    pub fq: Option<String>,
    /// default search field, example: weergavenaam
    pub df: Option<String>,
    /// query fields, example: weergavenaam^1.5,straatnaam^1.5
    pub bq: Option<String>,
    /// start index, default: 0
    pub start: Option<i32>,
    /// number of rows to return, default: 10, max: 100
    pub rows: Option<i32>,
    /// sort results on field, example: score desc,sortering asc,weergavenaam asc
    pub sort: Option<String>,
}

impl Default for FreeOptions {
    fn default() -> Self {
        Self {
            q: String::new(),
            best_match: Some(false),
            coordinates: None,
            fl: None,
            fq: None,
            df: None,
            bq: None,
            start: Some(0),
            rows: Some(10),
            sort: Some("score desc".to_string()),
        }
    }
}

impl FreeOptions {
    pub fn construct_query(&self) -> String {
        // create a mutable copy of self
        let mut options = self.clone();

        // if best match is true, overwrite params in options: rows to 1, sort on score desc, start on 0
        if let Some(true) = self.best_match {
            options.rows = Some(1);
            options.sort = Some("score desc".to_string());
            options.start = Some(0);
        }

        let mut query = format!("q={}", urlencoding::encode(&self.q));

        if let Some((longitude, latitude)) = self.coordinates {
            query.push_str(&format!("&lon={}", longitude));
            query.push_str(&format!("&lat={}", latitude));
        }

        if let Some(fl) = &options.fl {
            let fl = if fl.contains(',') {
                fl.split(',').collect::<Vec<&str>>().join("%2C")
            } else {
                fl.clone()
            };
            query.push_str(&format!("&fl={}", fl));
        }

        if let Some(fq) = &options.fq {
            query.push_str(&format!("&fq={}", fq));
        }

        if let Some(df) = &options.df {
            query.push_str(&format!("&df={}", df));
        }

        if let Some(bq) = &options.bq {
            // split on comma, every entry needs to be added as &bq=...
            let bq = bq.split(',').collect::<Vec<&str>>().join("&bq=");
            query.push_str(&format!("&bq={}", bq));
        }

        if let Some(start) = options.start {
            query.push_str(&format!("&start={}", start));
        }

        if let Some(rows) = options.rows {
            query.push_str(&format!("&rows={}", rows));
        }

        if let Some(sort) = &options.sort {
            query.push_str(&format!("&sort={}", sort));
        }

        query
    }
}
