mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use std::path::{Path, PathBuf};

pub use self::genpass::GenPassOpts;
pub use self::{
    base64::{Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64SubCommand},
    csv::{CsvOpts, OutputFormat},
    http::*,
    text::*,
};
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to other formats")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),

    #[command(subcommand, about = "Base64 encode or decode")]
    Base64(Base64SubCommand),

    #[command(subcommand, about = "Text sign or verify")]
    Text(TextSubCommand),

    #[command(subcommand, about = "HTTP server")]
    Http(HttpSubCommand),
}

// impl CmdExecutor for SubCommand {
//     async fn execute(self) -> anyhow::Result<()> {
//         match self {
//             SubCommand::Csv(opts) => opts.execute().await,
//             SubCommand::GenPass(opts) => opts.execute().await,
//             SubCommand::Base64(cmd) => cmd.execute().await,
//             SubCommand::Text(cmd) => cmd.execute().await,
//             SubCommand::Http(cmd) => cmd.execute().await,
//         }
//     }
// }

fn verify_file(filename: &str) -> Result<String, &'static str> {
    // Check if the file exists or input is "-"
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File does not exist"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("-").unwrap(), "-");
        assert!(verify_file("non_existent_file.csv").is_err());
    }
}
