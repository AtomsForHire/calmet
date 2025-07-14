use fitsio::FitsFile;
use fitsio::hdu::HduInfo;
use ndarray::{Zip, prelude::*};
use num_complex::Complex64;
use std::error::Error;
use std::path::PathBuf;

/// Struct for holding information about the calibration solutions
#[derive(Debug)]
pub(crate) struct Solutions {
    // Holds complex gains in XX, XY, YX, YY order
    pub(crate) complex_gains: Array3<Complex64>,

    // Holds MWA observation ID
    pub(crate) id: usize,

    // Number of tiles/stations
    pub(crate) num_tiles: usize,

    // Number of frequency channels
    pub(crate) num_chans: usize,
}

/// Struct for holding path to calibration solutions with methods for reading
pub(crate) struct SolutionsFile {
    pub(crate) file_path: PathBuf,
}

impl SolutionsFile {
    pub(crate) fn read_fits(&self) -> Result<Solutions, Box<dyn Error>> {
        let mut fptr = FitsFile::open(&self.file_path)?;

        let sol_hdu = fptr.hdu(1)?;

        let raw_data: ArrayD<f64> = sol_hdu.read_image(&mut fptr)?;
        let dim = raw_data.dim();

        let num_tiles = dim[1];
        let num_chans = dim[2];

        let real_view = raw_data.slice(s![.., .., .., ..;2]);
        let imag_view = raw_data.slice(s![.., .., .., 1..;2]);

        let complex_array = Zip::from(real_view)
            .and(imag_view)
            .map_collect(|&real, &imag| Complex64::new(real, imag));

        let complex_gains = complex_array.remove_axis(Axis(0));

        let id: i64 = fptr.hdu(0)?.read_key(&mut fptr, "OBSID")?;

        let result = Solutions {
            complex_gains: complex_gains,
            id: id as usize,
            num_tiles: num_tiles,
            num_chans: num_chans,
        };

        Ok(result)
    }
}
