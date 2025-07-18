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

    let num_tiles = solutions.num_tiles;

    let all_xx_complex_cal = solutions.complex_gains.slice(s![.., .., 0]);
    let all_yy_complex_cal = solutions.complex_gains.slice(s![.., .., 3]);

    // These should be shape [num_tiles, num_channels]
    let all_xx_gains = all_xx_complex_cal.map(|comp| comp.norm());
    let all_yy_gains = all_yy_complex_cal.map(|comp| comp.norm());

    // Calculate the median of each CHANNEL through tiles
    // NOTE: Cloning because the quantile axis method shuffles in place
    let median_xx_gains =
        all_xx_gains
            .clone()
            .quantile_axis_skipnan_mut(Axis(0), n64(0.5), &Linear)?;
    let median_yy_gains =
        all_yy_gains
            .clone()
            .quantile_axis_skipnan_mut(Axis(0), n64(0.5), &Linear)?;

    let mut xx_smoothness_vec: Vec<f64> = vec![];
    let mut yy_smoothness_vec: Vec<f64> = vec![];
    // Loop over antennas
    for i in 0..num_tiles {
        let mut tile_xx_gains = all_xx_gains.slice(s![i, ..]).to_owned();
        let mut tile_yy_gains = all_yy_gains.slice(s![i, ..]).to_owned();

        // Skip if antenna is flagged
        if tile_xx_gains.is_all_nan() {
            continue;
        }

        // Normalise by the median
        tile_xx_gains.zip_mut_with(&median_xx_gains, |x, &y| *x /= y);
        tile_yy_gains.zip_mut_with(&median_yy_gains, |x, &y| *x /= y);

        let xx_smooth = calculate_smoothness(&mut tile_xx_gains)?;
        let yy_smooth = calculate_smoothness(&mut tile_yy_gains)?;

        xx_smoothness_vec.push(xx_smooth);
        yy_smoothness_vec.push(yy_smooth);
    }
    Ok((solutions.id, xx_smoothness_vec, yy_smoothness_vec))
}

/// Caluclate gain smoothness with the FT
fn calculate_smoothness(gains: &mut Array1<f64>) -> Result<f64, Box<dyn Error>> {
    // Interpolate NaN gaps linearly
    gains.interp_nans_inplace();
    let num_chans = gains.len();

    // Convert to complex for ndfft() function
    let gains_complex = gains.map(|&gain| Complex64::new(gain, 0.0));

    let mut output = Array1::<Complex64>::zeros(num_chans);

    let mut handler: FftHandler<f64> = FftHandler::new(num_chans);
    ndfft(&gains_complex, &mut output, &mut handler, 0);

    let smooth_array = output.slice(s![1..num_chans / 2]).mapv(|x| x.norm()) / output[0].norm();
    let smooth = smooth_array.mean().expect("Unable to calculate smoothness");

    Ok(smooth)
}
