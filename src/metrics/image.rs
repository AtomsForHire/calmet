use crate::io::read::image::ImageFile;
use ndarray::prelude::*;
use ndarray_stats::QuantileExt;
use num_complex::ComplexFloat;
use std::{error::Error, path::Path};

pub(crate) fn run_image_calc(path: &Path) -> Result<(usize, f64, f64), Box<dyn Error>> {
    let file = ImageFile {
        file_path: path.to_path_buf(),
    };

    let image = file.read_fits()?;
    let rms = calc_rms(&image.data)?;
    let max = calc_max(&image.data)?;
    let dr = calc_dr(max, rms);
    Ok((image.id, rms, dr))
}

fn calc_rms(data: &Array2<f64>) -> Result<f64, Box<dyn Error>> {
    let result = data.powi(2).mean().unwrap().sqrt();
    Ok(result)
}

fn calc_dr(max: f64, rms: f64) -> f64 {
    max / rms
}

fn calc_max(data: &Array2<f64>) -> Result<f64, Box<dyn Error>> {
    Ok(*data.max()?)
}
