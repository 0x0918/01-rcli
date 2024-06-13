use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_http_serve,
    process_text_generate, process_text_sign, process_text_verify, Base64Subcommand,
    HttpSubcommand, Opts, Subcommand, TextSignFormat, TextSubcommand,
};
use std::fs::*;
use zxcvbn::zxcvbn;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }

        Subcommand::Genpass(opts) => {
            let pass = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("{}", pass);
            let estimate = zxcvbn(&pass, &[]);
            eprintln!("password strength: {}", estimate.score());
        }

        Subcommand::Base64(subcmd) => match subcmd {
            Base64Subcommand::Decode(opts) => {
                let decode = process_decode(&opts.input, opts.format)?;
                let decode = String::from_utf8(decode)?;
                println!("{}", decode);
            }
            Base64Subcommand::Encode(opts) => {
                let encode = process_encode(&opts.input, opts.format)?;
                println!("{}", encode);
            }
        },

        Subcommand::Text(subcmd) => match subcmd {
            TextSubcommand::Sign(opts) => {
                let sign = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", sign);
            }

            TextSubcommand::Verify(opts) => {
                let verified = process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
                println!("{}", verified);
            }
            TextSubcommand::Generate(opts) => {
                let key = process_text_generate(opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        write(name, &key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = &opts.output;
                        write(name.join("ed25519.sk"), &key[0])?;
                        write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
        },
        Subcommand::Http(cmd) => match cmd {
            HttpSubcommand::Serve(opts) => {
                process_http_serve(opts.dir, opts.port).await?;
            }
        },
    }
    Ok(())
}
