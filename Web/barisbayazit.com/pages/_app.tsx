import "../styles/globals.css"

import Head from "next/head"

import type { AppProps } from "next/app"

function App({ Component, pageProps }: AppProps) {
    return (
        <>
            <Head>
                <title>Barış Bayazıt</title>
            </Head>
            <Component {...pageProps} />
        </>
    )
}

export default App
