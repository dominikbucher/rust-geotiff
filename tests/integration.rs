extern crate geotiff as tiff;

use tiff::TIFF;

//#[test]
fn test_load() {
    match TIFF::open("resources/marbles.tif") {
        Ok(x) => println!("Read tiff {}", x),
        Err(e) => println!("File I/O Error: {:?}", e),
    }
}

#[test]
fn test_load_2() {
    match TIFF::open("resources/zh_dem_25.tif") {
        Ok(x) => {
            assert_eq!(x.image_data.len(), 366);
            assert_eq!(x.image_data[0].len(), 399);

            assert_eq!(x.get_value_at(0, 0), 551);
            assert_eq!(x.get_value_at(45, 67), 530);
            assert_eq!(x.get_value_at(142, 325), 587);
        },
        Err(e) => println!("File I/O Error: {:?}", e),
    }
}

// TODO Not supported yet, as this uses TileByteCounts instead of StripByteCounts.
//#[test]
fn test_load_3() {
    match TIFF::open("resources/large_tif/DEM_ZH.tif") {
        Ok(x) => {
            assert_eq!(x.image_data.len(), 366);
            assert_eq!(x.image_data[0].len(), 399);

            assert_eq!(x.get_value_at(0, 0), 551);
            assert_eq!(x.get_value_at(45, 67), 530);
            assert_eq!(x.get_value_at(142, 325), 587);
        },
        Err(e) => println!("File I/O Error: {:?}", e),
    }
}
