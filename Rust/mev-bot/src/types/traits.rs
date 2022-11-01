use async_trait::async_trait;
use ethers::{prelude::Transaction, types::Address};

#[async_trait]
pub trait Strategy {
    async fn on_new_block(&mut self);
    fn is_target(&self, address: Address) -> bool;
    fn is_profitable(&self, tx: &Transaction) -> bool;
    async fn execute(&self, tx: Transaction);
}
