import Navbar from "../components/Navbar"
import Footer from "../components/Footer"

import Link from "next/link"
import Image from "next/image"

const NotFound = () => {
    return (
        <div className="overflow-scroll flex flex-col min-h-screen justify-between bg-gray-100 dark:bg-black">
            <Navbar />
            <div className="flex justify-center flex-col p-8 h-full m-auto">
                <Image
                    src="/images/not_available.png"
                    alt="XKCD Comic - 404 #1969"
                    width={404}
                    height={404}
                />
                <Link href="/">
                    <a className="flex flex-row space-y-4 text-sm m-auto text-blue-400 underline select-none cursor-pointer">
                        Back to Home
                    </a>
                </Link>
            </div>
            <Footer />
        </div>
    )
}

export default NotFound
