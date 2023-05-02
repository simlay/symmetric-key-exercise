use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long)]
    key: String,

    #[structopt(short, long)]
    message: String,

    #[structopt(short, long, parse(from_os_str), default_value = "data.dat")]
    output: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
}
