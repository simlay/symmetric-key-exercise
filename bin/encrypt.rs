use structopt::StructOpt;

use symmetric_key_exercise::{
    SimpleCipherError,
    encrypt,
    NONCE_LENGTH,
    CommonEncryptionOpts,
};

#[derive(StructOpt, Debug)]
struct EncryptOpt {

    #[structopt(short, long)]
    message: String,

    #[structopt(flatten)]
    shared: CommonEncryptionOpts,
}

fn main() -> Result<(), SimpleCipherError> {
    let opt = EncryptOpt::from_args();
    Ok(())
}
