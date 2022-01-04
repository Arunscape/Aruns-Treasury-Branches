import React from 'react'
import Layout from '../../components/layout'

const FAQ: React.FC = () => {

    const cards = [
        {
            q: "How do I deposit diamonds?",
            a: "See transfers"
        },
        {
            q: "When will trading be available?",
            a: "Soon™"
        },
        {
            q: "Why should I trust you with my hard-earned diamonds?",
            a: "The code is 100% open source and is free for anyone to audit."
        }, {
            q: "How do I deposit diamonds?",
            a: "See transfers"
        },
        {
            q: "When will trading be available?",
            a: "Soon™"
        },
        {
            q: "Why should I trust you with my hard-earned diamonds?",
            a: "The code is 100% open source and is free for anyone to audit."
        }, {
            q: "How do I deposit diamonds?",
            a: "See transfers"
        },
        {
            q: "When will trading be available?",
            a: "Soon™"
        },
        {
            q: "Why should I trust you with my hard-earned diamonds?",
            a: "The code is 100% open source and is free for anyone to audit."
        }, {
            q: "How do I deposit diamonds?",
            a: "See transfers"
        },
        {
            q: "When will trading be available?",
            a: "Soon™"
        },
        {
            q: "Why should I trust you with my hard-earned diamonds?",
            a: "The code is 100% open source and is free for anyone to audit."
        }, {
            q: "How do I deposit diamonds?",
            a: "See transfers"
        },
        {
            q: "When will trading be available?",
            a: "Soon™"
        },
        {
            q: "Why should I trust you with my hard-earned diamonds?",
            a: "The code is 100% open source and is free for anyone to audit."
        }, {
            q: "How do I deposit diamonds?",
            a: "See transfers"
        },
        {
            q: "When will trading be available?",
            a: "Soon™"
        },
        {
            q: "Why should I trust you with my hard-earned diamonds?",
            a: "The code is 100% open source and is free for anyone to audit."
        }, {
            q: "How do I deposit diamonds?",
            a: "See transfers"
        },
        {
            q: "When will trading be available?",
            a: "Soon™"
        },
        {
            q: "Why should I trust you with my hard-earned diamonds?",
            a: "The code is 100% open source and is free for anyone to audit."
        }, {
            q: "How do I deposit diamonds?",
            a: "See transfers"
        },
        {
            q: "When will trading be available?",
            a: "Soon™"
        },
        {
            q: "Why should I trust you with my hard-earned diamonds?",
            a: "The code is 100% open source and is free for anyone to audit."
        },
    ];

    return <Layout>
        <section className="relative py-16 bg-white min-w-screen animation-fade animation-delay">
            <div className="container px-0 px-8 mx-auto sm:px-12 xl:px-5">
                <p className="text-xs font-bold text-left text-green-400 uppercase sm:mx-6 sm:text-center sm:text-normal sm:font-bold">
                    Got a Question? We’ve got answers.
                </p>
                <h3 className="mt-1 text-2xl font-bold text-left text-gray-800 sm:mx-6 sm:text-3xl md:text-4xl lg:text-5xl sm:text-center sm:mx-0">
                    Frequently Asked Questions
                </h3>
                {
                    cards.map(c => <div className="w-full px-6 py-6 mx-auto mt-10 bg-white border border-gray-200 rounded-lg sm:px-8 md:px-12 sm:py-8 sm:shadow lg:w-5/6 xl:w-2/3">
                        <h3 className="text-lg font-bold text-green-400 sm:text-xl md:text-2xl">
                            {c.q}
                        </h3>
                        <p className="mt-2 text-base text-gray-600 sm:text-lg md:text-normal">
                            {c.a}
                        </p>
                    </div>)
                }

            </div>
        </section>
    </Layout>
}

export default FAQ