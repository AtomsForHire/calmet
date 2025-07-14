use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
#[clap(arg_required_else_help = true)]
pub(super) struct ImgArgs {
    #[arg(short, long)]
    files: Vec<PathBuf>,
}
