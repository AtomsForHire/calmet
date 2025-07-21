mod cal_args;
mod img_args;
use crate::metrics::{gain_amplitude, gain_phase, image};

use crate::io::write::{write_results, write_results_1D};
use clap::{Parser, Subcommand};
use glob::glob;
use itertools::Itertools;
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
        match self {
            Commands::ImgMetrics(args) => run_img_metrics(args),
            Commands::CalMetrics(args) => run_cal_metrics(args),
            Commands::AmpMetrics(args) => run_amp_metrics(args),
            Commands::PhaseMetrics(args) => run_phase_metrics(args),
        }
    }
}

fn run_img_metrics(args: &img_args::ImgArgs) -> Result<(), Box<dyn Error>> {
    println!("Calculating image RMS and dynamic range");
    let paths = resolve_paths(&args.files)?;

    let (obsids, rms_vec, dr_vec): (Vec<_>, Vec<_>, Vec<_>) = paths
        .iter()
        .map(|path| image::run_image_calc(path))
        .filter_map(Result::ok)
        .multiunzip();

    write_results_1D(Path::new("image_rms.txt"), &obsids, &rms_vec)?;
    write_results_1D(Path::new("image_dr.txt"), &obsids, &dr_vec)?;
    Ok(())
}

fn run_cal_metrics(args: &cal_args::CalArgs) -> Result<(), Box<dyn Error>> {
    println!("Calculating amplitude smoothness, phase RMSE, and phase average euclidean distance");
    let paths = resolve_paths(&args.files)?;

    let (obsids, mut xx_smooth_vecs, mut yy_smooth_vecs): (Vec<_>, Vec<_>, Vec<_>) = paths
        .iter()
        .map(|path| gain_amplitude::run_smoothness_calc(path))
        .filter_map(Result::ok)
        .multiunzip();

    let (_, mut dist_res_vecs, mut xx_rmse_vecs, mut yy_rmse_vecs): (
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
    ) = paths
        .iter()
        .map(|path| gain_phase::run_phase_calcs(path))
        .filter_map(Result::ok)
        .multiunzip();

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
    Ok(())
}

fn run_amp_metrics(args: &cal_args::CalArgs) -> Result<(), Box<dyn Error>> {
    println!("Calculating amplitude smoothness");
    let paths = resolve_paths(&args.files)?;

    let (obsids, mut xx_smooth_vecs, mut yy_smooth_vecs): (Vec<_>, Vec<_>, Vec<_>) = paths
        .iter()
        .map(|path| gain_amplitude::run_smoothness_calc(path))
        .filter_map(Result::ok)
        .multiunzip();

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
    Ok(())
}

fn run_phase_metrics(args: &cal_args::CalArgs) -> Result<(), Box<dyn Error>> {
    println!("Calculating RMSE and average euclidean distance");
    let paths = resolve_paths(&args.files)?;

    let (obsids, mut dist_res_vecs, mut xx_rmse_vecs, mut yy_rmse_vecs): (
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
    ) = paths
        .iter()
        .map(|path| gain_phase::run_phase_calcs(path))
        .filter_map(Result::ok)
        .multiunzip();

    write_results(Path::new("xx_phase_rmse.txt"), &obsids, &mut xx_rmse_vecs)?;
    write_results(Path::new("yy_phase_rmse.txt"), &obsids, &mut yy_rmse_vecs)?;
    write_results(
        Path::new("euclidean_distance.txt"),
        &obsids,
        &mut dist_res_vecs,
    )?;
    Ok(())
}

fn resolve_paths(files: &[PathBuf]) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut input_files: Vec<PathBuf> = vec![];

    if files.len() == 1 {
        if let Some(path_str) = files[0].to_str() {
            if path_str.contains('*') {
                let paths = glob(path_str).map_err(|e| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid glob pattern: {}", e),
                    )
                })?;

                input_files = paths
                    .filter_map(Result::ok)
                    .filter(|p| p.is_file())
                    .collect();
            } else {
                let path = PathBuf::from(path_str);
                if path.is_file() {
                    input_files.push(path);
                }
            }
        }
    } else {
        input_files = files.iter().filter(|p| p.is_file()).cloned().collect();
    }

    if input_files.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No valid input files found.",
        ));
    }
    Ok(input_files)
}
