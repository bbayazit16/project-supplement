import DarkThemeHandler from "../../components/DarkThemeHandler"

import Link from "next/link"

const Navbar = () => {
    return (
        <nav className="overflow-scroll flex ml-4 mr-4 pl-12 pr-12 h-1/6 space-x-4 select-none">
            <DarkThemeHandler />
            <div className="w-full flex justify-end items-center space-x-8 sm:ml-[5%] sm:mr-[5%] md:ml-[15%] md:mr-[15%]">
                <Link href="/">
                    <a>Home</a>
                </Link>
                <Link href="/about">
                    <a>About Me</a>
                </Link>
            </div>
        </nav>
    )
}

export default Navbar
