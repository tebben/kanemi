//#[cfg(feature = "blosc")]
//use hdf5::filters::blosc_set_nthreads;
use hdf5::File;
use hdf5::Result;

pub fn read_hdf5(file_path: String) -> Result<()> {
    let file = File::open(file_path)?;

    /*     let group_geographic = file.group("geographic")?;
    let group_projection = group_geographic.group("map_projection")?; */

    /* let geo_row_offset = group_geographic.attr("geo_row_offset")?;
    let geo_number_rows = group_geographic.attr("geo_number_rows")?;
    let geo_number_columns = group_geographic.attr("geo_number_columns")?;
    let geo_proj4_string = group_projection.attr("projection_proj4_params");

    use ndarray::{arr2, Array2}; */

    // if error print the error

    // print the value

    //let attr_test = geo_proj4_string.unwrap();
    //let size = attr_test.storage_size();
    //let attr_value = attr_test.read_raw::<hdf5::types::FixedAscii<77>>();

    /*     if attr_value.is_err() {
        // print the actual error
        println!("Error: {:?}", attr_value.err());
        return Ok(());
    }

    let test = attr_value.unwrap();

    println!("TEST: {:?}", test); */

    /*     let ds = file.dataset("dir/pixels")?; // open the dataset
    assert_eq!(
        // read a slice of the 2-D dataset and verify it
        ds.read_slice::<Pixel, _, _>(s![1.., ..])?,
        arr2(&[
            [Pixel::new(3, 4, G), Pixel::new(4, 5, R)],
            [Pixel::new(5, 6, B), Pixel::new(6, 7, G)],
        ])
    );
    let attr = ds.attr("colors")?; // open the attribute
    assert_eq!(attr.read_1d::<Color>()?.as_slice().unwrap(), &[R, G, B]); */
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_hdf5() {
        read_hdf5("./example_data/example.hdf5".to_string()).unwrap();
    }
}
