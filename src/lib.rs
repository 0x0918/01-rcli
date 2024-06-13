mod cli;
mod process;
mod utils;

use anyhow::Result;
pub use cli::{
    Base64Format, Base64Subcommand, HttpSubcommand, Opts, Subcommand, TextSignFormat,
    TextSubcommand,
};
pub use process::*;
pub use utils::*;

#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(&self) -> Result<()>;
}
