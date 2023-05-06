use clap::Parser;

use symmetric_key_exercise::CommonEncryptionOpts;

#[derive(Parser, Debug)]
struct DecryptOpt {
    #[command(flatten)]
    shared: CommonEncryptionOpts,
}

fn main() -> anyhow::Result<()> {
    let opt = DecryptOpt::parse();
    let plaintext = opt.shared.decrypt()?;
    println!("{plaintext}");
    Ok(())
}
