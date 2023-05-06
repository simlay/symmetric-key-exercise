use clap::Parser;

use symmetric_key_exercise::CommonEncryptionOpts;

#[derive(Parser, Debug)]
struct EncryptOpt {
    #[arg(short, long)]
    /// The message to be encrypted.
    message: String,

    #[command(flatten)]
    shared: CommonEncryptionOpts,
}

fn main() -> anyhow::Result<()> {
    let opt = EncryptOpt::parse();
    let nonce = opt.shared.encrypt(opt.message)?;
    if let Some(nonce) = nonce {
        println!("The nonce for this message was generated and it is: {nonce}");
    }
    Ok(())
}
