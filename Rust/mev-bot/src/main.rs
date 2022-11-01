mod bot;
mod logger;
mod macros;
mod multicall;
mod strategy;
mod types;

use bot::builder::BotBuilder;
use logger::logger::info;
use strategy::uniswap_v2::UniswapV2;

use ethers::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

// build_address!("0xE592427A0AEce92De3Edee1F18E0157C05861564"), // v3_1
// build_address!("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45"), // v3_2 Auto Router
// .ws_client("wss://speedy-nodes-nyc.moralis.io/REDACTED/eth/mainnet/ws")
// .http_client("https://eth-mainnet.alchemyapi.io/v2/REDACTED")
// .ws_client("wss://eth-mainnet.alchemyapi.io/v2/REDACTED")

// .http_client("https://mainnet.infura.io/v3/REDACTED")
// .ws_client("wss://mainnet.infura.io/ws/v3/REDACTED")
#[tokio::main]
async fn main() {
    let graphql = 
    let bot = BotBuilder::default()
        .http_client("http://127.0.0.1:8545")
        .ws_client("ws://127.0.0.1:8546")
        .http_client("https://mainnet.infura.io/v3/REDACTED")
        .ws_client("wss://mainnet.infura.io/ws/v3/REDACTED")
        .signer("REDACTED")
        .bundle_signer("REDACTED")
        .build()
        .await;

    let ws_client = bot.ws_client();

    let bot = bot
        .strategy(
            UniswapV2::new(
                Arc::clone(&ws_client),
                build_address!("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f"), // Uni v2 factory
                build_address!("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D"), // Uni v2 router
            )
            .await,
        )
        .strategy(
            UniswapV2::new(
                Arc::clone(&ws_client),
                build_address!("0xC0AEe478e3658e2610c5F7A4A2E1777cE9e4f2Ac"), // Sushi factory
                build_address!("0xd9e1cE17f2641f24aE83637ab66a2cca9C378B9F"), // Sushi router
            )
            .await,
        )
        .build();

    let bot = Arc::new(Mutex::new(bot));

    let sa = bot.lock().await.signer_address();
    let bs = bot.lock().await.bundle_signer_address();

    info(
        format!(
            "Bot initialized with signer = {} bundle signer = {}",
            sa, bs
        )
        .as_str(),
    );
    drop(sa);
    drop(bs);

    tokio::spawn({
        let bot = Arc::clone(&bot);
        let tb = bot.lock().await;
        let mut stream = tb.watch_blocks().await;

        async move {
            while let Some(block) = stream.next().await {
                info("stratupdate skdaknfkanfa");
                bot.lock().await.update_strategies().await;
                info("heeee");
                let block_num = bot
                    .lock()
                    .await
                    .get_block(block)
                    .await
                    .number
                    .unwrap_or_default();
                info(format!("Updated strategies at block {}", block_num,).as_str());
            }
        }
    })
    .await
    .unwrap();

    tokio::spawn({
        let bot = Arc::clone(&bot);

        async move {
            let mut stream = bot.new_pending_transactions().await;

            while let Some(hash) = stream.next().await {
                let bot = Arc::clone(&bot);

                tokio::spawn(async move {
                    let tx = bot.get_transaction(hash).await;
                    if tx.is_some() && bot.is_pending(tx.as_ref().unwrap()) {
                        bot.match_tx(tx.unwrap()).await;
                    }
                });
            }
        }
    })
    .await
    .unwrap();
}
