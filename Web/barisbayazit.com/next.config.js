/** @type {import('next').NextConfig} */

module.exports = {
    reactStrictMode: true,
    swcMinify: true,
    trailingSlash: true,
    assetPrefix: "./",
    experimental: {
        images: {
            unoptimized: true,
        },
    },
}
