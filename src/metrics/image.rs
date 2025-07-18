use crate::io::read::image::ImageFile;
use ndarray::prelude::*;
use std::{error::Error, path::Path};

pub(crate) fn run_image_calc(path: &Path) -> Result<(), Box<dyn Error>> {
    let file = ImageFile {
        file_path: path.to_path_buf(),
    };

    file.read_fits()?;
    Ok(())
}

fn calc_rms() {}

fn calc_dr() {}
