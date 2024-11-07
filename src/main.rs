use clap::Parser;
// rcli csv -i input.csv -o output.json --header -d ','
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_text_sign,
    Base64SubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand,
};

fn main() -> anyhow::Result<()> {
    // opts will finally receive Opts::cmd which is SubCommand::Csv(CsvOpts)
    // which is the struct of CsvOpts
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => match opts.format {
                TextSignFormat::Blake3 => process_text_sign(&opts.input, &opts.key, opts.format)?,
                TextSignFormat::Ed25519 => {
                    println!("Ed25519");
                }
            },
            TextSubCommand::Verify(opts) => {
                println!("{:?}", opts);
            }
        },
    }

    Ok(())
}
