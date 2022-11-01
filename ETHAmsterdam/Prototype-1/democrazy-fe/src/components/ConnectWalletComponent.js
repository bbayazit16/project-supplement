import {Chip} from "@mui/material";
import FaceIcon from '@mui/icons-material/Face';
import {useWeb3React} from "@web3-react/core";
import Button from "@mui/material/Button";
import {Injected, WalletConnectConfig} from "../constants/WalletConnectConfig";

export function ConnectWalletComponent() {
  const { activate, deactivate,active, account } = useWeb3React();
  console.log(active,account);

  const walletButtons = (
    <div style={{
      marginBottom: -25
    }}>
      <Button variant="contained" onClick={ async(e) => {
        let provider = await activate(WalletConnectConfig);
      }}>
        WalletConnect
      </Button>
      <br /><Button variant="outlined" style={{margin:20, marginTop:10}} onClick={async()=>{
        let provider = await activate(Injected);
      }
      }>Metamask</Button>
    </div>
  )
  const connectedChip = <Chip onClick={deactivate} variant="outlined" color="success" sx={{ backgroundColor: "whitesmoke"}} icon={<FaceIcon />} label={"@"+account?.substring(0,15)+"..."}/>;
  const disconnectedChip = <Chip variant="outlined" color="warning" icon={<FaceIcon />} label={"Connected @"+account?.substring(0,6)+"..."}/>;
  return account ? (active?connectedChip : disconnectedChip) : walletButtons;
}
