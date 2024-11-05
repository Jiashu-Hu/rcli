mod cli;
mod process;

pub use cli::{Opts, SubCommand, OutputFormat, Base64SubCommand, Base64Format};
pub use process::{process_csv, process_genpass, process_encode, process_decode};