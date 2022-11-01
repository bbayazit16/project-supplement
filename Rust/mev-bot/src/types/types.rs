use ethers::prelude::{LocalWallet, Provider, SignerMiddleware, Ws};
use ethers_flashbots::FlashbotsMiddleware;

pub type FlashbotsClient =
    SignerMiddleware<FlashbotsMiddleware<Provider<Ws>, LocalWallet>, LocalWallet>;
