import { useState, useEffect } from "react"

import { connect } from "../utils/connectwallet"

import styles from "../styles/Home.module.css"
import SideBar from "../components/SideBar"
import DaoList from "../components/DaoList"
import DaoGrid from "../components/DaoGrid"

// const provider = new WalletConnectProvider({
//     rpc: {
//         69: process.env.ALCHEMY_URL,
//     },
// })r

const cardProps = {
    name: "PadawanDAO",
    subtitle: "13K members",
    img_url: "https://pbs.twimg.com/profile_images/1457753127004803080/WuTiPLFA_400x400.jpg",
}

const cardArray = [cardProps, cardProps, cardProps, cardProps, cardProps, cardProps]

export default function Home() {
    const [account, setAccount] = useState({
        address: null,
        ENS: null,
        displayAddress: null,
        profilePhoto: null,
    })

    useEffect(() => {
        if (window.ethereum) {
            window.ethereum.on("accountsChanged", accounts => {
                if (accounts.length === 0) {
                    localStorage.removeItem("connection")
                    window.location.reload()
                } else {
                    connect(localStorage.getItem("connection"))
                }
            })
            window.ethereum.on("disconnect", () => {
                localStorage.clear()
                window.location.reload()
            })
        }
        // Avoid spamming event listeners
        // eslint-disable-next-line
    })

    useEffect(() => {
        const type = localStorage.getItem("connection")
        if (type) {
            connect(type, setAccount)
        }
    }, [])

    useEffect(() => {
        // if (!account.address) {
        //     window.location.reload()
        // }
    }, [account.address])

    return (
        <div className="h-full bg-background-primary">
            <SideBar acc={account} handler={setAccount} />
            <DaoList />
        </div>
    )
}
