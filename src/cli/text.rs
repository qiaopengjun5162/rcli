use std::{fmt, path::PathBuf, str::FromStr};

use clap::Parser;
use enum_dispatch::enum_dispatch;
use tokio::fs;

use crate::{
    get_content, get_reader, process::process_text_key_generate, process_text_sign,
    process_text_verify, CmdExecutor,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

use super::{verify_file, verify_path};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum TextSubCommand {
    #[command(about = "Sign a text with a private/session key and return a signature")]
    Sign(TextSignOpts),

    #[command(about = "Verify a signature with a public/session key")]
    Verify(TextVerifyOpts),

    #[command(about = "Generate a random blake3 key or ed25519 key pair")]
    Generate(KeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg( long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg( long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(short, long)]
    pub sig: String,
}

#[derive(Debug, Parser)]
pub struct KeyGenerateOpts {
    #[arg(short, long, value_parser = parse_format, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output_path: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format: {}", s)),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExecutor for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let sig = process_text_sign(&mut reader, &key, self.format)?;
        // base64 output
        let encoded = URL_SAFE_NO_PAD.encode(sig);
        println!("{}", encoded);
        Ok(())
    }
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let decoded = URL_SAFE_NO_PAD.decode(&self.sig)?;
        let verified = process_text_verify(&mut reader, &key, &decoded, self.format)?;
        if verified {
            println!("✓ Signature verified");
        } else {
            println!("⚠ Signature not verified");
        }
        Ok(())
    }
}

impl CmdExecutor for KeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = process_text_key_generate(self.format)?;
        for (k, v) in key {
            fs::write(self.output_path.join(k), v).await?;
        }
        Ok(())
    }
}

// impl CmdExecutor for TextSubCommand {
//     async fn execute(self) -> anyhow::Result<()> {
//         match self {
//             TextSubCommand::Generate(opts) => opts.execute().await,
//             TextSubCommand::Sign(opts) => opts.execute().await,
//             TextSubCommand::Verify(opts) => opts.execute().await,
//         }
//     }
// }
