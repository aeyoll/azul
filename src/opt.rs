use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opt {
    /// Input folder
    #[structopt(short, long, parse(from_os_str))]
    pub input: PathBuf,

    /// Output file
    #[structopt(short, long, parse(from_os_str), default_value = "cover-mosaic.png")]
    pub output: PathBuf,

    /// The max size
    #[structopt(short = "s", long, default_value = "1000")]
    pub size: u32,
}
