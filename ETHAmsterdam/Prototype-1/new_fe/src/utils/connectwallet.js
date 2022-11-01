import WalletConnectProvider from "@walletconnect/ethereum-provider"
import { ethers } from "ethers"
import { createIcon } from "@download/blockies"

export const connect = async (type, handler) => {
    const ensprovider = new ethers.providers.JsonRpcProvider("https://rpc.ankr.com/eth")

    if (type === "injected") {
        if (window.ethereum) {
            const accounts = await window.ethereum.request({
                method: "eth_requestAccounts",
            })

            const address = ethers.utils.getAddress(accounts[0])

            const ENS = await ensprovider.lookupAddress(address)

            handler({
                address: address,
                ENS: ENS,
                displayAddress:
                    address.substring(0, 6) + "..." + address.substring(address.length - 4),
                profilePhoto: createIcon({
                    seed: address.toLowerCase(),
                    size: 8,
                    scale: 16,
                }).toDataURL("image/png"),
            })
            localStorage.setItem("connection", "injected")
        } else {
            alert("Injected provider not found!")
        }
    } else if (type === "walletconnect") {
        const provider = new WalletConnectProvider()

        try {
            await provider.enable()
        } catch {
            return
        }

        window.oldEthereum = window.ethereum

        window.ethereum = provider

        const accounts = await window.ethereum.request({
            method: "eth_requestAccounts",
        })

        const address = ethers.utils.getAddress(accounts[0])

        const ENS = await ensprovider.lookupAddress(address)

        handler({
            address: address,
            ENS: ENS,
            displayAddress: address.substring(0, 6) + "..." + address.substring(address.length - 4),
            profilePhoto: createIcon({
                seed: address.toLowerCase(),
                size: 8,
                scale: 16,
            }).toDataURL("image/png"),
        })
        localStorage.setItem("connectionType", "walletconnect")
    }
}
