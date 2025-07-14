use crate::io::read::SolutionsFile;
use crate::metrics::interp::InterpolateNans;
use ndarray::prelude::*;
use std::error::Error;
use std::path::Path;

pub(crate) fn run_phase_calcs(
    file_path: &Path,
) -> Result<(usize, Vec<f64>, Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let file = SolutionsFile {
        file_path: file_path.to_path_buf(),
    };
    let solutions = file.read_fits()?;

    let num_tiles = solutions.num_tiles;
    let num_chans = solutions.num_chans;
    let channels = Array1::<f64>::range(0.0, num_chans as f64, 1.0);

    let all_xx_complex_cal = solutions.complex_gains.slice(s![.., .., 0]);
    let all_yy_complex_cal = solutions.complex_gains.slice(s![.., .., 3]);

    // These should be shape [num_tiles, num_channels]
    let all_xx_angs = all_xx_complex_cal.map(|comp| comp.arg());
    let all_yy_angs = all_yy_complex_cal.map(|comp| comp.arg());

    let mut xx_rmse_vec: Vec<f64> = vec![];
    let mut yy_rmse_vec: Vec<f64> = vec![];
    let mut dist_vec: Vec<f64> = vec![];
    // Loop over antennas
    for i in 0..num_tiles {
        let tile_xx_angs = all_xx_angs.slice(s![i, ..]).to_owned();
        let tile_yy_angs = all_yy_angs.slice(s![i, ..]).to_owned();

        // 1. Calculate RMSE
        let mut x_fit = LinearRegression::new(channels.clone(), tile_xx_angs.clone());
        let mut y_fit = LinearRegression::new(channels.clone(), tile_yy_angs.clone());

        x_fit.fit();
        y_fit.fit();

        let x_rmse = x_fit.calc_rmse();
        let y_rmse = y_fit.calc_rmse();

        xx_rmse_vec.push(x_rmse);
        yy_rmse_vec.push(y_rmse);

        // 2. Calculate average euclidean distance
        let dist = calc_dist(&tile_xx_angs, &tile_yy_angs);
        dist_vec.push(dist);
    }

    Ok((solutions.id, dist_vec, xx_rmse_vec, yy_rmse_vec))
}

struct LinearRegression {
    x: Array1<f64>,
    y: Array1<f64>,
    pub gradient: Option<f64>,
    pub intercept: Option<f64>,
}

impl LinearRegression {
    /// Create new LinearRegression object
    fn new(x: Array1<f64>, y: Array1<f64>) -> Self {
        assert_eq!(x.len(), y.len());
        Self {
            x: x,
            y: y,
            gradient: None,
            intercept: None,
        }
    }

    /// Fit data and modify LinearRegression object
    fn fit(&mut self) {
        let n = self.x.len() as f64;

        if self.y.is_any_nan() {
            self.y.interp_nans_inplace();
        }

        // Calculate sums
        let sx: f64 = self.x.sum();
        let sy: f64 = self.y.sum();
        let sxx: f64 = self.x.powi(2).sum();
        let sxy: f64 = self
            .x
            .iter()
            .zip(self.y.iter())
            .map(|(&xi, &yi)| xi * yi)
            .sum();

        let m = (n * sxy - sx * sy) / (n * sxx - sx.powi(2));
        let c = (sy - m * sx) / n;

        self.gradient = Some(m);
        self.intercept = Some(c);
    }

    /// Calculat RMSE
    fn calc_rmse(&self) -> f64 {
        // Model data
        let yy = Array1::from_iter(
            self.x
                .iter()
                .map(|&e| e * self.gradient.unwrap() + self.intercept.unwrap()),
        );

        let result = ((self.y.clone() - yy).powi(2))
            .mean()
            .expect("Unable to calculate mean in RMSE")
            .sqrt();

        return result;
    }
}

fn calc_dist(pol1: &Array1<f64>, pol2: &Array1<f64>) -> f64 {
    assert_eq!(pol1.len(), pol2.len());

    // shift pol1 to start at pol2
    let shifted = pol1.clone() - (pol1[[0]] - pol2[[0]]);

    let result = (shifted - pol2)
        .mean()
        .expect("Unable to calculate mean in average euclidean distance")
        .abs();

    return result;
}
