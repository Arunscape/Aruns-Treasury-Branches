import type { NextPage } from 'next'
import Link from 'next/link'
import { useRouter } from 'next/router'

const Header: NextPage = () => {
    const { pathname } = useRouter();
    console.log(pathname === "/account")

    const underline = " underline underline-offset-4";

    return <section className="w-full px-8 text-gray-700 bg-white sticky top-0 z-50">
        <div className="container flex flex-col flex-wrap items-center justify-between py-5 mx-auto md:flex-row max-w-7xl">
            <div className="relative flex flex-col md:flex-row">
                <Link href="/">
                    <a className="flex items-center mb-5 font-medium text-gray-900 lg:w-auto lg:items-center lg:justify-center md:mb-0">
                        <span className="mx-auto text-xl font-black leading-none text-gray-900 select-none">
                            Arun's Treasury Branches
                            {/* Minecraft */}
                            {/* <span className="text-indigo-600">.</span> */}
                        </span>
                    </a>
                </Link>
                <nav className="flex flex-wrap items-center mb-5 text-base md:mb-0 md:pl-8 md:ml-8 md:border-l md:border-gray-200">
                    <Link href="/account">
                        <a
                            // href="#_"
                            className={"mr-5 font-medium leading-6 text-gray-600 hover:text-gray-900" + (pathname === "/account" ? underline : "")}
                        >
                            My Account
                        </a>
                    </Link>
                    <Link href="/transfers">
                        <a
                            // href="#_"
                            className={"mr-5 font-medium leading-6 text-gray-600 hover:text-gray-900"  + (pathname === "/transfers" ? underline : "")}
                        >
                            Transfers
                        </a>
                    </Link>
                    <Link href="/trade">
                        <a
                            // href="#_"
                            className={"mr-5 font-medium leading-6 text-gray-600 hover:text-gray-900" + (pathname === "/trade" ? underline : "")}
                        >
                            Trade
                        </a>
                    </Link>
                    <Link href="/invest">
                        <a
                            // href="#_"
                            className={"mr-5 font-medium leading-6 text-gray-600 hover:text-gray-900" + (pathname === "/invest" ? underline : "")}
                        >
                            Invest
                        </a>
                    </Link>
                    <Link href="/faq">
                        <a
                            // href="#_"
                            className={"mr-5 font-medium leading-6 text-gray-600 hover:text-gray-900" + (pathname === "/faq" ? underline : "")}
                        >
                            FAQ
                        </a>
                    </Link>
                </nav>
            </div>
            <div className="inline-flex items-center ml-5 space-x-6 lg:justify-end">
                <Link href="/signin">
                    <a
                        // href="#"
                        className="text-base font-medium leading-6 text-gray-600 whitespace-no-wrap transition duration-150 ease-in-out hover:text-gray-900"
                    >
                        Sign in
                    </a>
                </Link>
                <Link href="/signup">
                    <a
                        // href="#"
                        className="inline-flex items-center justify-center px-4 py-2 text-base font-medium leading-6 text-white whitespace-no-wrap bg-indigo-600 border border-transparent rounded-md shadow-sm hover:bg-indigo-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-600"
                    >
                        Sign up
                    </a>
                </Link>
            </div>
        </div>
    </section>
}


export default Header