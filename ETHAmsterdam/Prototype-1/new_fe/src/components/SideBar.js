import ConnectWallet from "./ConnectWallet"

const SideBar = ({ acc, handler }) => {
    return (
        <div className="flex-shrink">
            <div className="relative w-64 h-full">
                <div className="border-r-2 border-primary fixed h-screen flex flex-col w-64 bg-background-secondary shadow-md px-1 drop-shadow-md">
                    <div className="flex-1 flex flex-col pt-5 pb-4 overflow-y-auto items-center">
                        <div className="flex flex-shrink-0 px-4">
                            <img src="democrazy.svg" className="h-10 w-auto" />
                        </div>
                        <ConnectWallet acc={acc} handler={handler} />
                    </div>
                </div>
            </div>
        </div>
    )
}

export default SideBar
