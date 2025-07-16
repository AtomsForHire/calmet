mod cal_args;
use crate::metrics::{gain_amplitude, gain_phase};
mod img_args;

use crate::io::write::write_results;
use clap::{Parser, Subcommand};
use glob::glob;
use std::error::Error;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "calmet")]
#[command(about = "Calculates metrics from hyperdrive solutions")]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub(crate) sub_command: Commands,
}

#[derive(Subcommand)]
#[clap(arg_required_else_help = true)]
pub(super) enum Commands {
    #[clap(about = "Calculate all image metrics")]
    ImgMetrics(img_args::ImgArgs),

    #[clap(about = "Calculate all calibration metrics")]
    CalMetrics(cal_args::CalArgs),

    #[clap(about = "Calculate only EW and NS gain smoothness")]
    AmpMetrics(cal_args::CalArgs),

    #[clap(about = "Calculate only EW and NS phase metrics")]
    PhaseMetrics(cal_args::CalArgs),
}

impl Commands {
    pub(crate) fn run(&self) -> Result<(), Box<dyn Error>> {
        let files = match self {
            Commands::ImgMetrics(args) => &args.files,
            Commands::CalMetrics(args) => &args.files,
            Commands::AmpMetrics(args) => &args.files,
            Commands::PhaseMetrics(args) => &args.files,
        };

        let paths = resolve_paths(&files)?;
        match self {
            Commands::ImgMetrics(_) => {
                println!("Not yet implemented")
            }
            Commands::CalMetrics(_) => {
                println!(
                    "Calculating amplitude smoothness, phase RMSE, and phase average euclidean distance"
                );
                let mut obsids: Vec<usize> = vec![];
                let mut dist_res_vecs: Vec<Vec<f64>> = vec![];
                let mut xx_smooth_vecs: Vec<Vec<f64>> = vec![];
                let mut yy_smooth_vecs: Vec<Vec<f64>> = vec![];
                let mut xx_rmse_vecs: Vec<Vec<f64>> = vec![];
                let mut yy_rmse_vecs: Vec<Vec<f64>> = vec![];
                for path in paths.iter() {
                    let (_, xx_smooth, yy_smooth) = gain_amplitude::run_smoothness_calc(path)?;
                    xx_smooth_vecs.push(xx_smooth);
                    yy_smooth_vecs.push(yy_smooth);

                    let (id, dist, xx_rmse, yy_rmse) = gain_phase::run_phase_calcs(path)?;
                    xx_rmse_vecs.push(xx_rmse);
                    yy_rmse_vecs.push(yy_rmse);
                    dist_res_vecs.push(dist);

                    obsids.push(id);
                }

                write_results(
                    Path::new("xx_gain_smoothness.txt"),
                    &obsids,
                    &mut xx_smooth_vecs,
                )?;
                write_results(
                    Path::new("yy_gain_smoothness.txt"),
                    &obsids,
                    &mut yy_smooth_vecs,
                )?;
                write_results(Path::new("xx_phase_rmse.txt"), &obsids, &mut xx_rmse_vecs)?;
                write_results(Path::new("yy_phase_rmse.txt"), &obsids, &mut yy_rmse_vecs)?;
                write_results(
                    Path::new("euclidean_distance.txt"),
                    &obsids,
                    &mut dist_res_vecs,
                )?;
            }
            Commands::AmpMetrics(_) => {
                println!("Calculating amplitude smoothness");
                let mut obsids: Vec<usize> = vec![];
                let mut xx_smooth_vecs: Vec<Vec<f64>> = vec![];
                let mut yy_smooth_vecs: Vec<Vec<f64>> = vec![];
                for path in paths.iter() {
                    let (id, xx_res, yy_res) = gain_amplitude::run_smoothness_calc(path)?;
                    obsids.push(id);
                    xx_smooth_vecs.push(xx_res);
                    yy_smooth_vecs.push(yy_res);
                }

                write_results(
                    Path::new("xx_gain_smoothness.txt"),
                    &obsids,
                    &mut xx_smooth_vecs,
                )?;
                write_results(
                    Path::new("yy_gain_smoothness.txt"),
                    &obsids,
                    &mut yy_smooth_vecs,
                )?;
            }
            Commands::PhaseMetrics(_) => {
                println!("Calculating RMSE and average euclidean distance");
                let mut dist_res_vecs: Vec<Vec<f64>> = vec![];
                let mut obsids: Vec<usize> = vec![];
                let mut xx_rmse_vecs: Vec<Vec<f64>> = vec![];
                let mut yy_rmse_vecs: Vec<Vec<f64>> = vec![];
                for path in paths.iter() {
                    let (id, dist, xx_res, yy_res) = gain_phase::run_phase_calcs(path)?;
                    obsids.push(id);
                    xx_rmse_vecs.push(xx_res);
                    yy_rmse_vecs.push(yy_res);
                    dist_res_vecs.push(dist);
                }
                write_results(Path::new("xx_phase_rmse.txt"), &obsids, &mut xx_rmse_vecs)?;
                write_results(Path::new("yy_phase_rmse.txt"), &obsids, &mut yy_rmse_vecs)?;
                write_results(
                    Path::new("euclidean_distance.txt"),
                    &obsids,
                    &mut dist_res_vecs,
                )?;
            }
        }

        Ok(())
    }
}

fn resolve_paths(files: &Vec<PathBuf>) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut input_files: Vec<PathBuf> = vec![];
    let mut valid_files: bool = true;

    // If programs takes in a single input for input files, could be a wildcard input (depends on
    // shell) or just a single file
    if files.len() == 1 {
        if let Some(path_str) = files[0].to_str() {
            if path_str.contains("*") {
                match glob(path_str) {
                    Ok(paths) => {
                        for entry in paths {
                            match entry {
                                Ok(path) => {
                                    input_files.push(path);
                                }
                                Err(e) => {
                                    eprintln!("Unable to match glob: {}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Unable to match {} to files: {}", path_str, e);
                    }
                }
            } else {
                let path = PathBuf::from(path_str);
                if path.is_file() {
                    input_files.push(path);
                } else {
                    eprintln!("{:?} does not exist", path);
                }
            }
        } else {
            eprintln!("Could not convert '{:?}' to str", files[0]);
        }
    } else {
        for path in files.iter() {
            if path.is_file() {
                input_files.push(path.clone());
            } else {
                valid_files = false;
                break;
            }
        }
    }

    if valid_files && input_files.len() >= 1 {
        Ok(input_files)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "One of more input files are invalid",
        ))
    }
}
