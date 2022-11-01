import { connect } from "../utils/connectwallet"

const ConnectWallet = ({ acc, handler }) => {
    if (acc) {
        if (acc.address) {
            return (
                <button
                    onClick={() => {
                        localStorage.clear()
                        window.location.reload()
                    }}
                >
                    {"Connected @" + acc.ENS || acc.address.substring(0, 6) + "..."}
                </button>
            )
        }
        return (
            <div>
                <button
                    className="rounded-full"
                    onClick={async () => connect("walletconnect", handler)}
                >
                    WalletConnect
                </button>
                <br />
                <button className="rounded-full" onClick={async () => connect("injected", handler)}>
                    Metamask
                </button>
            </div>
        )
    }
    return (
        <div>
            <button
                className="rounded-full"
                onClick={async () => connect("walletconnect", handler)}
            >
                WalletConnect
            </button>
            <br />
            <button className="rounded-full" onClick={async () => connect("injected", handler)}>
                Metamask
            </button>
        </div>
    )

    // const connectButtons = (
    //     <div>
    //         <button
    //             className="rounded-full"
    //             onClick={async () => connect("walletconnect", handler)}
    //         >
    //             WalletConnect
    //         </button>
    //         <br />
    //         <button className="rounded-full" onClick={async () => connect("injected", handler)}>
    //             Metamask
    //         </button>
    //     </div>
    // )
    // const viewButtons = (
    //     <button
    //         onClick={() => {
    //             localStorage.clear()
    //             window.location.reload()
    //         }}
    //     >
    //         {"Connected @" + acc.address.substring(0, 6) + "..."}
    //     </button>
    // )

    // return acc ? (
    //     acc.address === null || acc.address === undefined ? (
    //         <div>
    //             <button
    //                 className="rounded-full"
    //                 onClick={async () => connect("walletconnect", handler)}
    //             >
    //                 WalletConnect
    //             </button>
    //             <br />
    //             <button className="rounded-full" onClick={async () => connect("injected", handler)}>
    //                 Metamask
    //             </button>
    //         </div>
    //     ) : (
    //         <button
    //             onClick={() => {
    //                 localStorage.clear()
    //                 window.location.reload()
    //             }}
    //         >
    //             {"Connected @" + acc.address.substring(0, 6) + "..."}
    //         </button>
    //     )
    // ) : (
    //     <div>
    //         <button
    //             className="rounded-full"
    //             onClick={async () => connect("walletconnect", handler)}
    //         >
    //             WalletConnect
    //         </button>
    //         <br />
    //         <button className="rounded-full" onClick={async () => connect("injected", handler)}>
    //             Metamask
    //         </button>
    //     </div>
    // )
    // const connectedChip = (
    //     <button onClick={deactivate}>
    //         {"Connected @" + account.address?.substring(0, 6) + "..."}
    //     </button>
    // )
    // const disconnectedChip = (
    //     <button>{"Connected @" + account.address?.substring(0, 6) + "..."}</button>
    // )
    // return account ? (active ? connectedChip : disconnectedChip) : walletButtons
}

export default ConnectWallet
