use std::fs;

use anyhow::Ok;
use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use rcli::{
    get_content, get_reader, process_csv, process_decode, process_encode, process_genpass,
    process_http_serve, process_text_key_generate, process_text_sign, process_text_verify,
    HttpSubCommand, Opts,
};
use zxcvbn::zxcvbn;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

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
        rcli::SubCommand::Genpass(opts) => {
            let ret = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("{}", ret);

            let estimate = zxcvbn(&ret, &[]);
            eprintln!("Password strength: {}", estimate.score());
        }
        rcli::SubCommand::Base64(subcmd) => match subcmd {
            rcli::Base64SubCommand::Encode(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let ret = process_encode(&mut reader, opts.format)?;
                println!("{}", ret);
            }
            rcli::Base64SubCommand::Decode(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let ret = process_decode(&mut reader, opts.format)?;
                println!("{}", ret);
            }
        },
        rcli::SubCommand::Text(cmd) => match cmd {
            rcli::TextSubCommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let sig = process_text_sign(&mut reader, &key, opts.format)?;
                let encoded = BASE64_URL_SAFE_NO_PAD.encode(sig);
                println!("{}", encoded);
            }
            rcli::TextSubCommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let decoded = BASE64_URL_SAFE_NO_PAD.decode(&opts.sig)?;
                let verified = process_text_verify(&mut reader, &key, &decoded, opts.format)?;
                if verified {
                    println!("signature verified ✅");
                } else {
                    println!("signature not verified ❌");
                }
            }
            rcli::TextSubCommand::Generate(opts) => {
                let key = process_text_key_generate(opts.format)?;
                for (k, v) in key {
                    fs::write(opts.output_path.join(k), v)?;
                }
            }
        },
        rcli::SubCommand::Http(cmd) => match cmd {
            HttpSubCommand::Serve(opts) => {
                process_http_serve(opts.dir, opts.port).await?;
            }
        },
    }
    Ok(())
}
