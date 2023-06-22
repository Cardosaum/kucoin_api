use anyhow::Result;
use clap::Parser;
use clap::Subcommand;
use futures::TryStreamExt;
use kucoin_api::client::Kucoin;
use kucoin_api::client::KucoinEnv;
use kucoin_api::model::websocket::KucoinWebsocketMsg;
use kucoin_api::model::websocket::WSTopic;
use kucoin_api::model::websocket::WSType;

#[derive(Parser, Debug)]
pub struct Websocket {
    #[command(subcommand)]
    cmd: WebSocketVariants,

    /// Fetch data requring authentication
    #[arg(long, global = true, conflicts_with = "public")]
    private: bool,

    /// Fetch data not requring authentication
    #[arg(
        long,
        global = true,
        conflicts_with = "private",
        default_value_t = true
    )]
    public: bool,
    // #[arg(short, long, value_enum)]
    // wstopic: WSTopic,
}

#[derive(Subcommand, Debug)]
pub enum WebSocketVariants {
    /// Get ticker for a symbol
    Ticker(Ticker),
}

#[derive(Parser, Debug)]
pub struct Ticker {
    symbol: String,
}

pub async fn process_websocket(cmd: Websocket) -> Result<()> {
    let kucoin = Kucoin::new(KucoinEnv::Live, None)?;
    match cmd.cmd {
        WebSocketVariants::Ticker(ticker) => {
            let url = kucoin.get_socket_endpoint(WSType::Public).await?;
            let mut ws = kucoin.websocket();
            ws.subscribe(url, vec![WSTopic::Ticker(vec![ticker.symbol])])
                .await?;

            while let Some(msg) = ws.try_next().await? {
                match msg {
                    KucoinWebsocketMsg::TickerMsg(msg) => println!("{msg}"),
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
