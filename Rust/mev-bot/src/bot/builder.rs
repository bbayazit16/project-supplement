use std::sync::Arc;

use super::bot::Bot;
use crate::logger::logger::fatal;
use crate::types::{traits::Strategy, types::FlashbotsClient};

use ethers::prelude::{Http, LocalWallet, Provider, SignerMiddleware, Ws};
use ethers_flashbots::FlashbotsMiddleware;
use url::Url;

pub struct BotBuilder {
    http_client: Option<String>,
    ws_client: Option<String>,
    signer: Option<String>,
    bundle_signer: Option<String>,
}

pub struct StrategyBuilder {
    http_client: Arc<Provider<Http>>,
    ws_client: Arc<Provider<Ws>>,
    signer: LocalWallet,
    flashbots_client: FlashbotsClient,
    strategies: Vec<Box<dyn Strategy + Send + Sync>>,
}

impl StrategyBuilder {
    pub fn ws_client(&self) -> Arc<Provider<Ws>> {
        Arc::clone(&self.ws_client)
    }

    pub fn strategy<T>(mut self, strategy: T) -> StrategyBuilder
    where
        T: Strategy + Send + Sync + 'static,
    {
        self.strategies.push(Box::new(strategy));
        self
    }

    pub fn build(self) -> Bot {
        Bot {
            http_client: self.http_client,
            ws_client: self.ws_client,
            signer: self.signer,
            flashbots_client: self.flashbots_client,
            strategies: self.strategies,
        }
    }
}

impl BotBuilder {
    pub fn default() -> BotBuilder {
        BotBuilder {
            http_client: None,
            ws_client: None,
            signer: None,
            bundle_signer: None,
        }
    }

    pub fn http_client(mut self, url: &str) -> BotBuilder {
        self.http_client = Some(url.to_owned());
        self
    }

    pub fn ws_client(mut self, url: &str) -> BotBuilder {
        self.ws_client = Some(url.to_owned());
        self
    }

    pub fn signer(mut self, key: &str) -> BotBuilder {
        self.signer = Some(key.to_owned());
        self
    }

    pub fn bundle_signer(mut self, key: &str) -> BotBuilder {
        self.bundle_signer = Some(key.to_owned());
        self
    }

    pub async fn build(self) -> StrategyBuilder {
        StrategyBuilder {
            http_client: Arc::new(
                Provider::<Http>::try_from(
                    self.http_client.unwrap_or_else(|| {
                        fatal("HTTP provider not specified");
                        panic!()
                    })[..]
                        .as_ref(),
                )
                .unwrap_or_else(|_| {
                    fatal("Unable to connect to HTTP provider");
                    panic!()
                }),
            ),
            ws_client: Arc::new(
                Provider::new(
                    Ws::connect(self.ws_client.as_ref().unwrap_or_else(|| {
                        fatal("Websocket url not specified");
                        panic!()
                    }))
                    .await
                    .unwrap_or_else(|_| {
                        fatal("Unable to connect to WS provider");
                        panic!()
                    }),
                )
                .interval(std::time::Duration::from_millis(1000)),
            ),
            signer: self
                .signer
                .as_ref()
                .unwrap_or_else(|| {
                    fatal("Signer not specified");
                    panic!()
                })
                .parse::<LocalWallet>()
                .unwrap_or_else(|_| {
                    fatal("Unable to parse signer");
                    panic!()
                }),
            flashbots_client: SignerMiddleware::new(
                FlashbotsMiddleware::new(
                    Provider::new(Ws::connect(self.ws_client.as_ref().unwrap()).await.unwrap())
                        .interval(std::time::Duration::from_millis(1000)),
                    Url::parse("https://relay.flashbots.net").unwrap_or_else(|_| {
                        fatal("Unable to connect to flashbots relay");
                        panic!()
                    }),
                    self.bundle_signer
                        .unwrap_or_else(|| {
                            fatal("Bundle signer not specified");
                            panic!()
                        })
                        .parse::<LocalWallet>()
                        .unwrap_or_else(|_| {
                            fatal("Unable to parse bundle signer");
                            panic!()
                        }),
                ),
                self.signer
                    .as_ref()
                    .unwrap()
                    .parse::<LocalWallet>()
                    .unwrap(),
            ),
            strategies: vec![],
        }
    }
}
