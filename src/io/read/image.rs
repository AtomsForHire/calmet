use fitsio::FitsFile;
use ndarray::prelude::*;
use std::{error::Error, path::PathBuf};

#[derive(Debug)]
pub(crate) struct Image {
    // Holds image data
    pub(crate) data: Array2<f64>,

    // Holds MWA observation ID
    pub(crate) id: usize,

    // Number of pixels in the x direction
    pub(crate) num_pixels_x: usize,

    // Number of pixels in the y direction
    pub(crate) num_pixels_y: usize,
}

pub(crate) struct ImageFile {
    pub(crate) file_path: PathBuf,
}

impl ImageFile {
    pub(crate) fn read_fits(&self) -> Result<Image, Box<dyn Error>> {
        // Check if file is fits
        if self.file_path.extension().and_then(|s| s.to_str()) != Some("fits") {
            // Do something
        }

        let stem = self
            .file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("Unable to get file's stem");

        let (gps, filler) = stem
            .split_once("_")
            .expect("Could not split image file name to get obsid");

        println!("{gps}");

        let mut fptr = FitsFile::open(&self.file_path)?;

        let img_hdu = fptr.hdu(0)?;

        // let id: i64 = img_hdu.read_key(&mut fptr, "OBSID")?;

        let raw_data: ArrayD<f64> = img_hdu.read_image(&mut fptr)?;
        let dim = raw_data.dim();
        assert_eq!(dim[0], 1);
        assert_eq!(dim[1], 1);
        let num_pixels_x = dim[2];
        let num_pixels_y = dim[3];

        let data = raw_data.slice_move(s![0, 0, .., ..]);

        let result = Image {
            data: data,
            id: 0,
            num_pixels_x: num_pixels_x,
            num_pixels_y: num_pixels_y,
        };

        Ok(result)
    }
}
