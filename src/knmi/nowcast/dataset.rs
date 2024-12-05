//#[cfg(feature = "blosc")]
//use hdf5::filters::blosc_set_nthreads;
use hdf5::{File, H5Type, Result};
use ndarray::Array;

const CALIBRATION_FACTOR: f64 = 0.01;
const GEO_ROW_OFFSET: f64 = 3649.98193359375;
const GEO_COLUMN_OFFSET: f64 = 0.0;
const GEO_NUMBER_OF_ROWS: u16 = 765;
const GEO_NUMBER_OF_COLUMNS: u16 = 700;
//const GEO_PIXEL_SIZE_X: f64 = 1.000003457069397;
//const GEO_PIXEL_SIZE_Y: f64 = 1.000004768371582;
/* const GEO_PRODUCT_CORNERS: [[f64; 2]; 4] = [
    [0.0, 49.362064361572266],
    [0.0, 55.973602294921875],
    [10.856452941894531, 55.388973236083984],
    [9.009300231933594, 48.895301818847656],
]; */

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

#[test]
fn test_read_hdf5() {
    read_hdf5("./example_data/test.hdf5".to_string()).unwrap();
}

#[test]
fn test_pixel_value_to_mm_hr() {
    assert_eq!(pixel_to_mm_hr(113), 13.56);
    assert_eq!(pixel_to_mm_hr(31), 3.72);
    assert_eq!(pixel_to_mm_hr(0), 0.00);
}

// get precipitation in mm/hr from pixel value rounded to 2 decimals
pub fn pixel_to_mm_hr(value: u8) -> f64 {
    let result = (value as f64 * CALIBRATION_FACTOR) * 12.0;
    (result * 100.0).round() / 100.0
}

pub fn read_hdf5(file: String) -> Result<()> {
    //use Color::*;
    //let file = File::open(file)?;

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

/* pub func open_dataset(file_path: &str) -> Dataset {
    Dataset {
        file_path: file_path.to_string(),
        row_offset: 0,
        number_of_rows: 0,
        number_of_columns: 0,
        proj4_string: "".to_string(),
    }
} */
