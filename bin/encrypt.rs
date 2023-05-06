use structopt::StructOpt;

use symmetric_key_exercise::CommonEncryptionOpts;

#[derive(StructOpt, Debug)]
struct EncryptOpt {
    #[structopt(short, long)]
    message: String,

    #[structopt(flatten)]
    shared: CommonEncryptionOpts,
}

fn main() -> anyhow::Result<()> {
    let opt = EncryptOpt::from_args();
    let nonce = opt.shared.encrypt(opt.message)?;
    if let Some(nonce) = nonce {
        println!("The nonce for this message was generated and it is: {nonce}");
    }
    Ok(())
}
