use hdf5::File;
use hdf5::Result;

pub fn read_hdf5(file_path: String) -> Result<()> {
    let file = File::open(file_path)?;
    print!("{:?}", file);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_hdf5() {
        read_hdf5("../example_data/example.hdf5".to_string()).unwrap();
    }
}
