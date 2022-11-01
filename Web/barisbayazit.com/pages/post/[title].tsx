import Navbar from "../../components/Navbar"
import Footer from "../../components/Footer"

import Link from "next/link"
import md from "markdown-it"
import matter from "gray-matter"
import hljs from "highlight.js"

import defineSolidityHighlighting from "highlightjs-solidity"
import useSuperscript from "markdown-it-sup"

defineSolidityHighlighting(hljs)

import { GetStaticPaths, GetStaticProps } from "next"
import { readdirSync, readFileSync } from "fs"

interface Post {
    title: string
    date: string
    lastEdit: string
    contents: string
}

const mdIt = md()
    .set({
        highlight: (code, lang) => {
            if (lang && hljs.getLanguage(lang)) {
                try {
                    return hljs.highlight(code, { language: lang }).value
                } catch {}
            }

            try {
                return hljs.highlightAuto(code).value
            } catch {}
        },
    })
    .use(useSuperscript)

mdIt.renderer.rules.link_open = (tokens, idx, options, _, self) => {
    const tok = tokens[idx].attrIndex("target")

    if (tok < 0) {
        tokens[idx].attrPush(["target", "_blank"])
    } else {
        tokens[idx].attrs[tok][1] = "_blank"
    }

    tokens[idx].attrSet("rel", "noopener noreferrer") // why not, just for good measure
    return self.renderToken(tokens, idx, options)
}

const Blog = ({ post }: { post: Post }) => {
    return (
        <div className="overflow-scroll flex flex-col min-h-screen justify-between bg-gray-100 dark:bg-black">
            <Navbar />
            <main className="flex flex-col space-y-2 p-8 h-full mb-auto mt-10 md:mt-16 ml-[5%] mr-[5%] sm:ml-[12.5%] sm:mr-[12.5%] md:ml-[25%] md:mr-[25%]">
                <h1 className="text-4xl font-bold">{post.title}</h1>
                <div className="flex flex-row">
                    <Link href="/">
                        <a className="text-blue-400 underline text-sm select-none cursor-pointer">
                            Back to Home
                        </a>
                    </Link>
                    <span className="text-gray-700 dark:text-gray-400 text-sm ml-2">
                        {post.date}
                        {post.lastEdit && ` • ${post.lastEdit}`}
                    </span>
                </div>
                <br className="select-none" />
                <article className="prose" dangerouslySetInnerHTML={{ __html: post.contents }} />
            </main>
            <Footer />
        </div>
    )
}

export const getStaticProps: GetStaticProps = ({ params }) => {
    const { title } = params

    const { data: context, content } = matter(readFileSync(`./posts/${title}.md`, "utf8"))

    return {
        props: {
            post: {
                title: context.title,
                date: context.date,
                lastEdit: context.lastEdit,
                contents: mdIt.render(content),
            },
        },
    }
}

export const getStaticPaths: GetStaticPaths = () => {
    return {
        paths: readdirSync("./posts/").map(articleName => {
            return {
                params: {
                    title: articleName.replace(".md", ""),
                },
            }
        }),
        fallback: false,
    }
}

export default Blog
