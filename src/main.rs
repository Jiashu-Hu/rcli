use std::os::unix::process;

use clap::Parser;
// rcli csv -i input.csv -o output.json --header -d ','
use rcli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    // opts will finally receive Opts::cmd which is SubCommand::Csv(CsvOpts)
    // which is the struct of CsvOpts
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            process_csv(&opts.input, &opts.output)?
        }
    }

    Ok(())
}