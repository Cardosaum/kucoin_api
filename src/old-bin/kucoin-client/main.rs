use anyhow::Result;

use clap::Parser;
use clap::Subcommand;

use market::process_market;
use market::Market;
use websocket::process_websocket;
use websocket::Websocket;

mod market;
mod websocket;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Get public market data
    #[command(subcommand)]
    Market(Market),
    /// Get real-time market data
    Websocket(Websocket),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
/// TODO: Write documentation
struct Args {
    /// List of subcommands
    #[command(subcommand)]
    cmd: Command,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    match args.cmd {
        Command::Market(market_command) => process_market(market_command).await?,
        Command::Websocket(websocket_command) => process_websocket(websocket_command).await?,
    }
    Ok(())
}
