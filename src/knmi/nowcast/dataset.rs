//#[cfg(feature = "blosc")]
//use hdf5::filters::blosc_set_nthreads;
use hdf5::{File, H5Type, Result};
use ndarray::Array;
//use ndarray::{arr2, s};

/* #[derive(H5Type, Clone, PartialEq, Debug)] // register with HDF5
#[repr(u8)]
pub enum Color {
    R = 1,
    G = 2,
    B = 3,
}

#[derive(H5Type, Clone, PartialEq, Debug)] // register with HDF5
#[repr(C)]
pub struct Pixel {
    xy: (i64, i64),
    color: Color,
} */

/* impl Pixel {
    pub fn new(x: i64, y: i64, color: Color) -> Self {
        Self { xy: (x, y), color }
    }
} */

// Not used yet
/* pub struct Dataset {
    file_path: String,
    row_offset: i32,
    number_of_rows: i32,
    number_of_columns: i32,
    proj4_string: String,
} */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_hdf5() {
        read_hdf5("./example_data/test.hdf5".to_string()).unwrap();
    }
}

//https://github.com/search?q=repo%3Ametno%2Fhdf5-rust

pub fn read_hdf5(file: String) -> Result<()> {
    //use Color::*;
    let file = File::open(file)?;

    let group_geographic = file.group("geographic")?;
    let group_projection = group_geographic.group("map_projection")?;

    let geo_row_offset = group_geographic.attr("geo_row_offset")?;
    let geo_number_rows = group_geographic.attr("geo_number_rows")?;
    let geo_number_columns = group_geographic.attr("geo_number_columns")?;
    let geo_proj4_string = group_projection.attr("projection_proj4_params").unwrap();

    use hdf5::types::DynValue;
    use hdf5::types::VarLenUnicode;
    use ndarray::{arr2, Array2};

    /*     let value = geo_row_offset.shape();
       let r: VarLenUnicode = geo_row_offset.as_reader().read_scalar().unwrap();
    */
    // get type of the attribute
    let attr_type = geo_proj4_string.dtype().unwrap();

    // this prints to a Attribute type: <HDF5 datatype>, i want to get the actual type
    println!("Attribute type: {:?}", attr_type);

    let value = geo_proj4_string.read_raw::<VarLenUnicode>();

    // if error print the error
    if value.is_err() {
        println!("Error: {:?}", value.err());
    } else {
        // print the value
        println!("TEST: {:?}", value.unwrap());
    }

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

/* pub func open_dataset(file_path: &str) -> Dataset {
    Dataset {
        file_path: file_path.to_string(),
        row_offset: 0,
        number_of_rows: 0,
        number_of_columns: 0,
        proj4_string: "".to_string(),
    }
} */
