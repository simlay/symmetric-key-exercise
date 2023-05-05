use structopt::StructOpt;

use symmetric_key_exercise::{CommonEncryptionOpts, SimpleCipherError};

#[derive(StructOpt, Debug)]
struct DecryptOpt {
    #[structopt(flatten)]
    shared: CommonEncryptionOpts,
}

fn main() -> Result<(), SimpleCipherError> {
    let opt = DecryptOpt::from_args();
    let plaintext = opt.shared.decrypt()?;
    println!("{plaintext}");
    Ok(())
}
