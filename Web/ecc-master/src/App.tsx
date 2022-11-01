// Library Imports
import {ReactElement, FC, useState, useEffect} from 'react';
import {createIcon} from '@download/blockies';
import {ethers} from 'ethers';
// Component Imports
import Button from './Components/Button';
import Contact from './Components/Contact';
import ChatBubble from './Components/ChatBubble';
// Type imports
import IAccount from './Types/IAccount';
import IMessage from './Types/IMesssage';
// Asset Imports
import ethercordlogo from './Assets/ethercordlogo.png';
import metamasklogo from './Assets/metamasklogo.svg';
import walletconnectlogo from './Assets/walletconnectlogo.svg';
import uploadicon from './Assets/upload.svg';
import sendicon from './Assets/send.svg';

// Import Gun.
import Gun from 'gun';
import Sea from 'gun/sea';
const gun = Gun();
const user = gun.user();
//

const ACCESS_MESSAGE = (address: string): string => {
  return 'Log in to Ethercord account ' + ethers.utils.keccak256(address);
};

const blockies = (addr: string): string => {
  return createIcon({
    seed: addr.toLowerCase(),
    size: 8,
    scale: 16
  }).toDataURL('image/png');
};

// 0xAAAA...bbbb
const shorten = (addr: string): string => {
  return addr.substring(0, 6) + '...' + addr.substring(addr.length - 4);
};

const fetchMetadata = async (account: IAccount): Promise<IAccount> => {
  // Create a new Ethers provider using ANKR public rpc.
  // If type is not set to any, getAvatar() function won't be
  // found.
  const provider: any = new ethers.providers.JsonRpcProvider('https://rpc.ankr.com/eth');

  const ens: string | null = await provider.lookupAddress(account.address);

  if (ens) {
    // Provider.getAvatar() from Ethers automatically looks up ENS metadata
    // and returns the ENS avatar of a given ENS name.
    const avatar: string | null = await provider.getAvatar(ens);

    // If ENS exists, set account properties.
    if (avatar) {
      return {
        address: account.address,
        displayName: ens,
        ENS: ens,
        avatar: avatar
      };
    }
    return {...account, displayName: ens, ENS: ens};
  }

  return account;
};

const App: FC = (): ReactElement => {
  //
  const [account, setAccount] = useState<IAccount | null>();

  const [showConnectWallet, setShowConnectWallet] = useState<boolean>(false);

  const connectWallet = async (connectionType: string): Promise<void> => {
    if (connectionType === 'metamask' || connectionType === 'walletconnect') {
      //
      let address: string;
      //
      if (connectionType === 'metamask' && window.ethereum) {
        // Choose the first account from the accounts access was given
        address = (await window.ethereum.request({method: 'eth_requestAccounts'}))[0];

        localStorage.setItem('connectionType', 'metamask');
      } else {
        //
        // TODO: walletconnect
        address = '';
      }

      // Complete address to checksum.
      // Every operation with addresses (including usernames on gun)
      // is done with checksummed addresses.
      address = ethers.utils.getAddress(address);

      setShowConnectWallet(true);

      // Set the account immediately with the generated profile photos.
      // The check for ENS address is done later with the fetchMetadata() function.
      // With this approach the user will see a placeholder until their metadata
      // is fetched from ENS (if exists).
      setAccount({
        address: address,
        displayName: shorten(address),
        avatar: blockies(address)
      });

      const pass: any = await window.ethereum.request({
        method: 'personal_sign',
        params: [address, ACCESS_MESSAGE(address)]
      });

      // Check if user has registered in the DB before.
      gun.get(`~@${address}`).once((data) => {
        // if user does not exist, data is undefined.
        if (!data) {
          // if the user has not logged in before
          user.create(address, pass, (acc) => {
            console.log(acc);
          });
        } else {
          //
          user.auth(address, pass, (acc: any) => {
            Sea.encrypt("hello", acc.sea, console.log)
          });
        }
      });
      return;
    }
  };

  // On wallet connect
  useEffect(() => {
    if (account) {
      fetchMetadata(account).then((acc) => {
        setAccount(acc);
      });
    }
    // eslint-disable-next-line
  }, [account?.address]);

  // On website load
  useEffect(() => {
    const connectionType: string | null = localStorage.getItem('connectionType');
    if (connectionType) {
      connectWallet(connectionType);
    }
  }, []);

  useEffect(() => {
    if (window.ethereum) {
      window.ethereum.on('accountsChanged', (accounts: string[]) => {
        if (accounts.length === 0) {
          // Disconnected
          setAccount(null);
          localStorage.removeItem('connectionType');
          setShowConnectWallet(false);
        } else {
          // Account "actually" changed, reconnect
          const connectionType: string | null = localStorage.getItem('connectionType');
          if (connectionType) {
            connectWallet(connectionType);
          }
        }
      });
      window.ethereum.on('disconnect', () => {
        setAccount(null);
        localStorage.removeItem('connectionType');
        setShowConnectWallet(false);
        // if (window.oldEthereum) {
        //   window.ethereum = window.oldEthereum;
        // }
      });
    }
    // eslint-disable-next-line
  }, [window.ethereum]);

  const m: IMessage = {
    message:
      'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.',
    from: '0x5ed565904ED1f093F12E8a1c465Fe1C8569E278F',
    to: '0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B'
  };

  const n: IMessage = {
    message:
      'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.',
    from: '0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B',
    to: '0x5ed565904ED1f093F12E8a1c465Fe1C8569E278F'
  };

  const chats = [m, n, n, m, m, m, m, n, n];

  return (
    <div className="pl-8 pr-8 pt-2 w-full h-screen overflow-auto">
      <div className="flex p-1 w-full rounded-xl border-2 border-slate-800">
        <div className="flex w-full">
          <div className="flex items-end space-x-16 w-full">
            <div className="flex space-x-2 items-center ml-4">
              <img
                src={ethercordlogo}
                alt="Ethercord"
                style={{height: '64px'}}
                className="hover:rotate-360 duration-1000 rotate-10"
              ></img>
              <p className="font-bold text-2xl select-none hidden md:block">Ethercord</p>
              <span className="animate-pulse text-red-100 select-none">Beta</span>
            </div>
            <div className="flex pl-2 pr-2 h-full justify-center items-center w-80 min-w-20">
              <input
                type="text"
                placeholder="Search for address or ENS..."
                className="rounded-lg bg-lapis-700 border-slate-700 border-2 w-full pl-1 select-none"
              ></input>
              <div className="flex ml-2 bg-slate-800 rounded-full w-12 justify-center items-center hover:scale-125 duration-1000">
                <span className="flex font-bold text-sm cursor-pointer select-none">Search</span>
              </div>
            </div>
          </div>
          <div className="flex flex-shrink-0 flex-nowrap items-center mx-4">
            <div className="flex items-center justify-center space-x-4">
              <Button text="Run a node" />
              {showConnectWallet ? (
                account?.address ? (
                  <Button
                    text={account.displayName}
                    image={account.avatar}
                    imageExtra="hover:rotate-360 duration-1000"
                    showBG={true}
                  />
                ) : (
                  <>
                    <Button
                      text="WalletConnect"
                      onClick={() => connectWallet('walletconnect')}
                      image={walletconnectlogo}
                      imageExtra="mt-1"
                      showBG={false}
                    />
                    <Button
                      text="Metamask"
                      onClick={() => connectWallet('metamask')}
                      image={metamasklogo}
                      showBG={false}
                    />
                  </>
                )
              ) : (
                <Button
                  text="Connect Wallet"
                  onClick={() => setShowConnectWallet(true)}
                  image={blockies('1984')}
                  showBG={true}
                />
              )}
            </div>
          </div>
        </div>
      </div>
      <div className="flex flex-row mt-4 h-5/6 ml-8 mr-8 overflow-auto">
        <div className="flex flex-col w-1/3 rounded-xl border-2 border-slate-800">
          <div className="flex-row w-full pr-2 pl-2 pt-1 pb-1 mt-2">
            <input
              type="text"
              placeholder="Search chats..."
              className="rounded-lg bg-lapis-700 border-slate-700 border-2 w-full pl-1 select-none"
            ></input>
          </div>
          <div className="mt-3 overflow-y-scroll scrollbar-hide">
            <div className="flex-col mx-2 mb-2 space-y-3">
              {/* TODO: Remove test vitalik's */}
              <Contact
                displayName="somereallylongensnameomgokthisissolong.eth"
                avatar={blockies('0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B')}
              />
              <Contact
                displayName="vitalik.eth"
                avatar={blockies('0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B')}
              />
              <Contact
                displayName="vitalik.eth"
                avatar={blockies('0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B')}
              />
              <Contact
                displayName="vitalik.eth"
                avatar={blockies('0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B')}
              />
              <Contact
                displayName="vitalik.eth"
                avatar={blockies('0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B')}
              />
              <Contact
                displayName="vitalik.eth"
                avatar={blockies('0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B')}
              />
              <Contact
                displayName="vitalik.eth"
                avatar={blockies('0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B')}
              />
              <Contact
                displayName="vitalik.eth"
                avatar={blockies('0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B')}
              />
              <Contact
                displayName="vitalik.eth"
                avatar={blockies('0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B')}
              />
            </div>
          </div>
        </div>
        <div className="flex flex-col h-full w-full ml-4 border-slate-800 border-2 rounded-xl bg-slate-900 overflow-hidden">
          <div className="flex items-center bg-lapis-800 border-slate-800 border-2 rounded-md">
            <div className="flex space-x-2 p-1 pl-4">
              <span>vitalik.eth - 0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B</span>
              <div className="rounded-full h-6 w-6 ml-1 overflow-hidden border-black border-4 bg-black">
                <img
                  src={blockies('0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B')}
                  alt="User Avatar"
                  className="select-none"
                ></img>
              </div>
            </div>
          </div>

          <div className="flex overflow-y-scroll scrollbar-hide h-full">
            <div className="flex mt-2">
              <div className="flex flex-col m-2 mt-0 w-full space-y-4 overflow-y-scroll scrollbar-hide">
                {
                  account
                    ? chats.map((c, i) => {
                        return (
                          <ChatBubble
                            message={c.message}
                            self={c.from === account.address}
                            key={i}
                          />
                        );
                      })
                    : null /* TODO: Introduction here */
                }
              </div>
            </div>
          </div>

          <div className="flex h-1/6 flex-row bg-lapis-900 p-3">
            <div className="flex items-center w-full mr-2">
              <textarea
                className="rounded-l-lg bg-lapis-700 border-slate-700 border-r-0 outline-none
            border-2 h-full w-full pl-1 select-none resize-none mb-1 text-lg leading-none -mr-1 scrollbar-hide"
              ></textarea>
              <div
                className="bg-lapis-700 border-slate-700 border-l-0 rounded-r-lg
                border-2 h-full pl-1 select-none resize-none mb-1  text-lg leading-none w-20"
              >
                <div
                  className="flex justify-center pt-2 pb-2 pl-1 pr-1 cursor-pointer w-full h-full
                border-lapis-700 border-1 rounded-r-lg object-cover space-x-2"
                >
                  {/* Invisible border makes images slightly smaller while preserving the optimal height and width*/}
                  <div className="flex h-full w-full border-lapis-700 border-2">
                    <img src={uploadicon} alt="Upload File"></img>
                  </div>
                  <div className="flex h-full w-full border-lapis-700 border-2">
                    <img src={sendicon} alt="Send"></img>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default App;
