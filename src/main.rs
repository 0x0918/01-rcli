use anyhow::Ok;
use clap::Parser;
use rcli::{process_csv, Opts};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        rcli::SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }
    Ok(())
}
