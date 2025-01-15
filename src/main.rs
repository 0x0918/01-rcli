use anyhow::Ok;
use clap::Parser;
use rcli::{process_csv, process_decode, process_encode, process_genpass, Opts};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        rcli::SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        rcli::SubCommand::Genpass(opts) => process_genpass(
            opts.length,
            opts.uppercase,
            opts.lowercase,
            opts.number,
            opts.symbol,
        )?,
        rcli::SubCommand::Base64(subcmd) => match subcmd {
            rcli::Base64SubCommand::Encode(opts) => process_encode(&opts.input, opts.format)?,
            rcli::Base64SubCommand::Decode(opts) => process_decode(&opts.input, opts.format)?,
        },
    }
    Ok(())
}
