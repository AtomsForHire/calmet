mod cal_args;
use crate::metrics::gain_amplitude;
mod img_args;

use crate::io::write::write_smoothness;
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
    #[clap(about = "Calculate all calibration metrics")]
    All(cal_args::CalArgs),

    #[clap(about = "Calculate only EW and NS gain smoothness")]
    GainSmoothness(cal_args::CalArgs),
}

impl Commands {
    pub(crate) fn run(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Commands::All(args) => {
                let files = args.files.clone();
                let paths = resolve_paths(&files)?;
            }
            Commands::GainSmoothness(args) => {
                let files = args.files.clone();
                let paths = resolve_paths(&files)?;
                let mut obsids: Vec<usize> = vec![];
                let mut xx_res_vecs: Vec<Vec<f64>> = vec![];
                let mut yy_res_vecs: Vec<Vec<f64>> = vec![];
                for path in paths.iter() {
                    let (id, xx_res, yy_res) = gain_amplitude::run_smoothness_calc(path)?;
                    obsids.push(id);
                    xx_res_vecs.push(xx_res);
                    yy_res_vecs.push(yy_res);
                }

                write_smoothness(Path::new("xx_gain_smoothness.txt"), &obsids, &xx_res_vecs)?;
                write_smoothness(Path::new("yy_gain_smoothness.txt"), &obsids, &yy_res_vecs)?;
            }
        }

        Ok(())
    }
}

fn run_gain_smoothness(paths: Vec<PathBuf>) {}

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
