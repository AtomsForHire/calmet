use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
#[clap(arg_required_else_help = true)]
pub(crate) struct CalArgs {
    #[arg(short, long, num_args=1..,)]
    pub(super) files: Vec<PathBuf>,
}
