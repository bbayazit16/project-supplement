use crate::logger::logger::*;
use crate::multicall::multicallwrapper::MulticallWrapper;
use crate::types::traits::Strategy;
use crate::{current_time, current_time_dur};

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::mpsc::channel;

use ethers::{
    abi::AbiDecode,
    prelude::*,
    types::{Address, Transaction},
};

abigen!(UniswapV2Factory, "src/abi/uni_v2_factory.json");
abigen!(UniswapV2Router, "src/abi/uni_v2_router.json");

pub struct UniswapV2 {
    provider: Arc<Provider<Ws>>,
    factory: Arc<UniswapV2Factory<Provider<Ws>>>,
    router: Address,
    pairs: Vec<Address>,
}

impl UniswapV2 {
    pub async fn new(
        provider: Arc<Provider<Ws>>,
        factory_address: Address,
        router: Address,
    ) -> UniswapV2 {
        let factory = Arc::new(UniswapV2Factory::new(
            factory_address,
            Arc::clone(&provider),
        ));
        let pairs = UniswapV2::get_all_reserves(Arc::clone(&provider), Arc::clone(&factory)).await;
        UniswapV2 {
            provider,
            router,
            factory,
            pairs,
        }
    }

    #[allow(dead_code)]
    async fn get_all_reserves(
        provider: Arc<Provider<Ws>>,
        factory: Arc<UniswapV2Factory<Provider<Ws>>>,
    ) -> Vec<Address> {
        let start = current_time_dur!();
        info("Fetching Uniswap V2 pairs...");

        let all_pairs_length: usize = factory
            .all_pairs_length()
            .call()
            .await
            .unwrap_or_else(|_| {
                fatal("Unable to fetch all pairs at Uniswap V2 Strategy");
                panic!();
            })
            .as_u32() as usize;

        let multicall_call_count: usize = 128;
        let thread_count: usize = 16;

        let mut multicall_count = all_pairs_length / multicall_call_count;
        if all_pairs_length % multicall_call_count != 0 {
            multicall_count += 1;
        }

        let mut multicalls_per_thread = multicall_count / thread_count;
        if multicall_count % thread_count != 0 {
            multicalls_per_thread += 1;
        }

        let (sx, mut rx) = channel(multicall_count);

        for i in 0..thread_count {
            tokio::spawn({
                let lower_bound = i * multicalls_per_thread;
                let upper_bound = lower_bound + multicalls_per_thread;
                let provider = Arc::clone(&provider);
                let factory = Arc::clone(&factory);
                let sx = sx.clone();
                async move {
                    let mut j = lower_bound * multicall_call_count;
                    while j < upper_bound * multicall_call_count {
                        let mut k = j;
                        let mut multicall: MulticallWrapper<Address> =
                            MulticallWrapper::new(Arc::clone(&provider));
                        while k < j + multicall_call_count && k < all_pairs_length {
                            multicall.add_call(factory.all_pairs(U256::from(k as u32)));
                            k += 1;
                        }
                        if multicall.len() == 0 {
                            if j < all_pairs_length {
                                fatal("Thread leak at Uniswap V2 Strategy");
                            }
                        } else {
                            j += multicall_call_count;
                            let pairs = multicall.call().await;
                            sx.send(pairs).await.unwrap();
                        }
                    }
                }
            });
        }

        let mut pairs = vec![];
        while let Some(mut pair_chunks) = rx.recv().await {
            pairs.append(&mut pair_chunks);
            if pairs.len() == all_pairs_length {
                break;
            }
        }
        let end = current_time_dur!();
        info(
            format!(
                "Done fetching {} Uniswap V2 Pairs in {:?}",
                all_pairs_length,
                end - start
            )
            .as_str(),
        );
        pairs
    }
}

#[async_trait]
impl Strategy for UniswapV2 {
    async fn on_new_block(&mut self) {
        self.pairs =
            UniswapV2::get_all_reserves(Arc::clone(&self.provider), Arc::clone(&self.factory))
                .await;
    }

    fn is_target(&self, address: Address) -> bool {
        if address == self.router {
            return true;
        }
        false
    }

    fn is_profitable(&self, tx: &Transaction) -> bool {
        let input = UniswapV2RouterCalls::decode(&tx.input);
        if input.is_err() {
            warn("Invalid tx input at Uniswap V2 Strategy");
            return false;
        }

        match input.unwrap() {
            UniswapV2RouterCalls::SwapExactETHForTokens(data) => {
                if current_time!() > data.deadline.as_u64() {
                    return false;
                }
                dbg!(data);
                true
            }
            UniswapV2RouterCalls::SwapETHForExactTokens(data) => {
                if current_time!() > data.deadline.as_u64() {
                    return false;
                }
                true
            }
            _ => false,
        }
    }

    async fn execute(&self, tx: Transaction) {
        info(format!("Potential tx = {:?}", tx.hash).as_str());
        // dbg!(tx);
    }
}
