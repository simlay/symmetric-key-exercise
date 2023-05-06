use structopt::StructOpt;

use symmetric_key_exercise::CommonEncryptionOpts;

#[derive(StructOpt, Debug)]
struct DecryptOpt {
    #[structopt(flatten)]
    shared: CommonEncryptionOpts,
}

fn main() -> anyhow::Result<()> {
    let opt = DecryptOpt::from_args();
    let plaintext = opt.shared.decrypt()?;
    println!("{plaintext}");
    Ok(())
}
