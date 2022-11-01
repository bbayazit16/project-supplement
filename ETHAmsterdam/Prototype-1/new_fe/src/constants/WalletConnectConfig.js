import {WalletConnectConnector} from "@web3-react/walletconnect-connector";
import {InjectedConnector} from "@web3-react/injected-connector";
export const WalletConnectConfig = new WalletConnectConnector({
  rpcUrl: `https://mainnet.infura.io/v3/${process.env.ALCHEMY_URL}`,
  bridge: "https://bridge.walletconnect.org",
  qrcode: true,
});
export const Injected = new InjectedConnector({
  supportedChainIds: [69]
});
