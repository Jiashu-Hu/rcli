use clap::Parser;
// rcli csv -i input.csv -o output.json --header -d ','
use rcli::{process_csv, Opts, SubCommand};

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
    }

    Ok(())
}