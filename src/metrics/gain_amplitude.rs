use crate::io::read::solutions::CalSolFile;
use crate::metrics::interp::InterpolateNans;
use ndarray::prelude::*;
use ndarray_stats::QuantileExt;
use ndarray_stats::interpolate::Linear;
use ndrustfft::{FftHandler, ndfft};
use noisy_float::types::n64;
use num_complex::Complex64;
use std::error::Error;
use std::path::Path;

/// Wrapper around the actual smoothnes calculation
pub(crate) fn run_smoothness_calc(
    file_path: &Path,
) -> Result<(usize, Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let file = CalSolFile {
        file_path: file_path.to_path_buf(),
    };
    let solutions = file.read_fits()?;

    let mut all_xx_gains = solutions
        .complex_gains
        .slice(s![.., .., 0])
        .map(|c| c.norm());
    let mut all_yy_gains = solutions
        .complex_gains
        .slice(s![.., .., 3])
        .map(|c| c.norm());

    // Need to clone since quantile_axis_skipnan_mut mutates arrays in place.
    let median_xx_gains =
        all_xx_gains
            .clone()
            .quantile_axis_skipnan_mut(Axis(0), n64(0.5), &Linear)?;
    let median_yy_gains =
        all_yy_gains
            .clone()
            .quantile_axis_skipnan_mut(Axis(0), n64(0.5), &Linear)?;

    let (xx_smoothness_vec, yy_smoothness_vec) = all_xx_gains
        .axis_iter_mut(Axis(0))
        .zip(all_yy_gains.axis_iter_mut(Axis(0)))
        .filter(|(xx, _)| !xx.is_all_nan()) // Skip if flagged antenna
        .map(|(mut xx, mut yy)| {
            xx.zip_mut_with(&median_xx_gains, |x, &y| *x /= y);
            yy.zip_mut_with(&median_yy_gains, |y, &z| *y /= z);
            (
                calculate_smoothness(&mut xx.to_owned()).unwrap(),
                calculate_smoothness(&mut yy.to_owned()).unwrap(),
            )
        })
        .unzip();

    Ok((solutions.id, xx_smoothness_vec, yy_smoothness_vec))
}

/// Caluclate gain smoothness with the FT
fn calculate_smoothness(gains: &mut Array1<f64>) -> Result<f64, Box<dyn Error>> {
    gains.interp_nans_inplace();
    let num_chans = gains.len();

    let mut complex_gains = Array1::from_iter(gains.iter().map(|&g| Complex64::new(g, 0.0)));
    let mut output = Array1::<Complex64>::zeros(num_chans);
    let mut handler = FftHandler::new(num_chans);

    ndfft(&mut complex_gains, &mut output, &mut handler, 0);

    let smooth_array = output.slice(s![1..num_chans / 2]).mapv(|x| x.norm()) / output[0].norm();
    Ok(smooth_array.mean().expect("Unable to calculate smoothness"))
}
