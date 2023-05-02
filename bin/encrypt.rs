use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long)]
    _key: String,

    #[structopt(short, long)]
    _message: String,

    #[structopt(short, long, parse(from_os_str), default_value = "data.dat")]
    _output: PathBuf,
}

fn main() {
    let _opt = Opt::from_args();
}
