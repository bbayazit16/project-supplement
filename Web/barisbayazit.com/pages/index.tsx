import Navbar from "../components/Navbar"
import Footer from "../components/Footer"

import Link from "next/link"
import matter from "gray-matter"

import type { NextPage, GetStaticProps } from "next"
import { readdirSync, readFileSync } from "fs"

interface PostContext {
    title: string
    id: string
    date: string
    lastEdit: string
}

const Home: NextPage = ({ posts }: { posts: PostContext[] }) => {
    return (
        <div className="overflow-scroll flex flex-col min-h-screen justify-between bg-gray-100 dark:bg-black">
            <Navbar />
            <main className="flex flex-col p-8 h-full mb-auto mt-10 ml-[5%] mr-[5%] sm:ml-[12.5%] sm:mr-[12.5%] md:ml-[25%] md:mr-[25%]">
                <div className="flex flex-col space-y-4 mb-4">
                    <p className="text-2xl font-bold" translate="no">
                        Barış Bayazıt's Website
                    </p>
                    <p className="text-md">Below you can find some of my blog posts.</p>
                </div>
                <div className="border-t-4 border-gray-400 dark:border-gray-600" />
                <br className="select-none" />
                <div className="flex flex-col space-y-8 ">
                    {posts.map(
                        post =>
                            post.id.charAt(0) !== "_" && (
                                <div key={post.id} className="flex flex-col">
                                    <Link href={`/post/${post.id}`}>
                                        <div className="">
                                            <a className="text-blue-400 underline text-md dark:text-blue-400 inline cursor-pointer">
                                                {post.title}
                                            </a>
                                        </div>
                                    </Link>
                                    <span className="text-gray-700 dark:text-gray-400 text-sm">
                                        {post.date}
                                        {post.lastEdit && ` • ${post.lastEdit}`}
                                    </span>
                                </div>
                            )
                    )}
                </div>
            </main>
            <Footer />
        </div>
    )
}

export const getStaticProps: GetStaticProps = () => {
    return {
        props: {
            posts: readdirSync("./posts/").map(article => {
                const { data: context } = matter(readFileSync(`./posts/${article}`, "utf8"))
                return {
                    title: context.title,
                    id: article.replace(".md", ""),
                    date: context.date,
                    lastEdit: context.lastEdit,
                }
            }),
        },
    }
}

export default Home
