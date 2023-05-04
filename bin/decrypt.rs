use structopt::StructOpt;

use symmetric_key_exercise::{
    SimpleCipherError,
    encrypt,
    NONCE_LENGTH,
    CommonEncryptionOpts,
};

#[derive(StructOpt, Debug)]
struct DecryptOpt {
    #[structopt(flatten)]
    shared: CommonEncryptionOpts,

}

fn main() -> Result<(), SimpleCipherError> {
    let opt = DecryptOpt::from_args();
    Ok(())
}
