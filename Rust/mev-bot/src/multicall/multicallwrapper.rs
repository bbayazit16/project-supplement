use crate::build_address;
use crate::logger::logger::fatal;

use ethers::{
    abi::{Detokenize, Function},
    prelude::{abigen, builders::ContractCall, NameOrAddress, Provider, Ws},
    types::{Address, Bytes},
};
use std::{fmt::Debug, marker::PhantomData, sync::Arc};

abigen!(MulticallContract, "src/abi/multicall.json");

/// MulticallWrapper wraps MakerDAO's Multicall contract and removes
/// the 15 call limitation by ethers-rs. All outputs are expected to
/// be the same type.
pub struct MulticallWrapper<D> {
    _marker: PhantomData<D>,
    contract: MulticallContract<Provider<Ws>>,
    targets: Vec<(Address, Bytes)>,
    func: Option<Function>,
}

impl<D: Detokenize> MulticallWrapper<D> {
    pub fn new(provider: Arc<Provider<Ws>>) -> MulticallWrapper<D> {
        MulticallWrapper {
            _marker: PhantomData,
            contract: MulticallContract::new(
                build_address!("0xeefBa1e63905eF1D7ACbA5a8513c70307C1cE441"),
                provider,
            ),
            targets: vec![],
            func: None,
        }
    }

    pub fn add_call(&mut self, call: ContractCall<Provider<Ws>, D>) {
        if let NameOrAddress::Address(address) = call.tx.to().unwrap() {
            self.targets.push((*address, call.calldata().unwrap()));
            if self.func.is_none() {
                self.func = Some(call.function);
            }
        } else {
            fatal("Multicall does not accept ENS");
        }
    }

    pub fn len(&self) -> usize {
        self.targets.len()
    }

    pub async fn call(self) -> Vec<D> {
        let results: Vec<Bytes> = self
            .contract
            .aggregate(self.targets)
            .call()
            .await
            .unwrap_or_else(|_| {
                fatal("Multicall failed");
                panic!();
            })
            .1;
        if self.func.is_none() {
            fatal("No calls provided!");
        }
        let func = self.func.unwrap();
        let mut ret: Vec<D> = vec![];
        for byte in results {
            let output = func.decode_output(byte.as_ref()).unwrap_or_else(|_| {
                fatal("Unable to decode multicall output");
                panic!();
            });
            ret.push(D::from_tokens(output).unwrap_or_else(|_| {
                fatal("Invalid output type provided to multicall");
                panic!();
            }));
        }
        ret
    }
}

impl<D> Debug for MulticallWrapper<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "len = {}", self.targets.len())
    }
}

impl<D> Clone for MulticallWrapper<D> {
    fn clone(&self) -> Self {
        MulticallWrapper {
            _marker: PhantomData,
            contract: self.contract.clone(),
            targets: vec![],
            func: None,
        }
    }
}
