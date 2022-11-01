use crate::logger::logger::*;
use crate::types::{traits::Strategy, types::FlashbotsClient};

use ethers::prelude::*;
use std::sync::Arc;

pub struct Bot {
    #[allow(dead_code)]
    pub(crate) http_client: Arc<Provider<Http>>,
    pub(crate) ws_client: Arc<Provider<Ws>>,
    pub(crate) signer: LocalWallet,
    pub(crate) flashbots_client: FlashbotsClient,
    pub(crate) strategies: Vec<Box<dyn Strategy + Send + Sync>>,
}

impl Bot {
    pub async fn match_tx(&self, tx: Transaction) {
        for strategy in &self.strategies {
            if strategy.is_target(tx.to.unwrap_or(H160::zero())) && strategy.is_profitable(&tx) {
                strategy.execute(tx).await;
                break;
            }
        }
    }

    pub async fn update_strategies(&mut self) {
        for strategy in self.strategies.iter_mut() {
            strategy.on_new_block().await;
        }
    }

    pub async fn watch_blocks(&self) -> FilterWatcher<'_, Ws, TxHash> {
        self.ws_client.watch_blocks().await.unwrap_or_else(|_| {
            fatal("Unable to listen to block updates");
            panic!();
        })
    }

    pub async fn new_pending_transactions(&self) -> SubscriptionStream<'_, Ws, H256> {
        self.ws_client
            .subscribe_pending_txs()
            .await
            .unwrap_or_else(|_| {
                fatal("Unable to listen to pending transactions");
                panic!()
            })
    }

    pub async fn get_transaction(&self, hash: TxHash) -> Option<Transaction> {
        let tx = self.ws_client.get_transaction(hash).await;
        if tx.is_err() {
            warn("Unable to fetch tx");
        };
        tx.unwrap_or_default()
    }

    pub async fn get_block(&self, block: H256) -> Block<TxHash> {
        self.ws_client
            .get_block(block)
            .await
            .unwrap_or_default()
            .unwrap_or_default()
    }

    pub fn is_pending(&self, tx: &Transaction) -> bool {
        tx.block_number.is_none()
    }

    pub fn signer_address(&self) -> String {
        format!("{:?}", self.signer.address())
    }

    pub fn bundle_signer_address(&self) -> String {
        format!("{:?}", self.flashbots_client.address())
    }
}
