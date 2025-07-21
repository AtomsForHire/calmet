use crate::io::read::solutions::CalSolFile;
use crate::metrics::interp::InterpolateNans;
use itertools::Itertools;
use ndarray::prelude::*;
use std::error::Error;
use std::path::Path;

pub(crate) fn run_phase_calcs(
    file_path: &Path,
) -> Result<(usize, Vec<f64>, Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let file = CalSolFile {
        file_path: file_path.to_path_buf(),
    };
    let solutions = file.read_fits()?;
    let channels = Array1::<f64>::range(0.0, solutions.num_chans as f64, 1.0);

    let all_xx_angs = solutions
        .complex_gains
        .slice(s![.., .., 0])
        .map(|c| c.arg());
    let all_yy_angs = solutions
        .complex_gains
        .slice(s![.., .., 3])
        .map(|c| c.arg());

    let (dist_vec, xx_rmse_vec, yy_rmse_vec): (Vec<_>, Vec<_>, Vec<_>) = all_xx_angs
        .axis_iter(Axis(0))
        .zip(all_yy_angs.axis_iter(Axis(0)))
        .map(|(xx_angs, yy_angs)| {
            let mut x_fit = LinearRegression::new(channels.clone(), xx_angs.to_owned());
            x_fit.fit();
            let mut y_fit = LinearRegression::new(channels.clone(), yy_angs.to_owned());
            y_fit.fit();

            (
                calc_dist(&xx_angs, &yy_angs),
                x_fit.calc_rmse(),
                y_fit.calc_rmse(),
            )
        })
        .collect::<Vec<_>>()
        .into_iter()
        .multiunzip();

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

        let sx = self.x.sum();
        let sy = self.y.sum();
        let sxx = self.x.powi(2).sum();
        let sxy = (&self.x * &self.y).sum();

        let m = (n * sxy - sx * sy) / (n * sxx - sx.powi(2));
        let c = (sy - m * sx) / n;

        self.gradient = Some(m);
        self.intercept = Some(c);
    }

    /// Calculat RMSE
    fn calc_rmse(&self) -> f64 {
        let yy = &self.x * self.gradient.unwrap() + self.intercept.unwrap();
        ((&self.y - &yy).powi(2))
            .mean()
            .expect("Unable to calculate mean in RMSE")
            .sqrt()
    }
}

fn calc_dist(pol1: &ArrayView1<f64>, pol2: &ArrayView1<f64>) -> f64 {
    // shift pol1 to start at pol2
    let shifted = pol1 - (pol1[0] - pol2[0]);
    (shifted - pol2)
        .mean()
        .expect("Unable to calculate mean in average euclidean distance")
        .abs()
}
