mod cli;
mod process;
mod utils;

pub use cli::{
    Base64Format, Base64SubCommand, HttpSubCommand, Opts, SubCommand, TextSignFormat,
    TextSubCommand,
};
pub use process::{
    process_csv, process_decode, process_encode, process_genpass, process_http_serve,
    process_text_key_generate, process_text_sign, process_text_verify,
};
pub use utils::*;

#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
