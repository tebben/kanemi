use crate::harmonie_cy43_p1::reader::CY43P1Reader;
use crate::harmonie_cy43_p1::reader::GribError;
use crate::harmonie_cy43_p1::reader::GribResponse;

#[derive(Debug)]
pub struct Dataset {
    pub filepaths: Vec<String>,
    pub readers: Vec<CY43P1Reader>,
}

impl Dataset {
    pub fn new(filepaths: Vec<String>) -> Result<Dataset, GribError> {
        let mut readers = vec![];
        for filepath in &filepaths {
            let reader = CY43P1Reader::open(filepath)?;
            readers.push(reader);
        }

        Ok(Dataset { filepaths, readers })
    }

    pub fn get(
        &self,
        parameters: Option<Vec<(&str, u16)>>,
        locations: Option<Vec<(f32, f32)>>,
    ) -> Result<Vec<GribResponse>, GribError> {
        let mut result = vec![];
        for reader in &self.readers {
            let p = parameters.clone();
            let l = locations.clone();
            let reader_result = reader.get(p, l)?;
            result.push(reader_result);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE_PATH1: &str = "../example_data/HA43_N20_202412221800_00000_GB";

    #[test]
    fn test_dataset_new() {
        let parameters = vec![("tmp", 0), ("isba", 802)];
        let locations = vec![(5.351926, 51.716_8), (4.913082420058467, 52.3422859189378)];
        let filepaths = vec![FILE_PATH1.to_string()];
        let dataset = Dataset::new(filepaths).unwrap();

        let data = dataset.get(Some(parameters), Some(locations)).unwrap();

        let pretty_data = serde_json::to_string_pretty(&data).unwrap();
        println!("{}", pretty_data);
    }
}
