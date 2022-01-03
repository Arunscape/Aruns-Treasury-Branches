import Link from 'next/link'
import Head from 'next/head'
import Layout from '../../components/layout'
const AccountPage = () => (
    <Layout>
        <Head>
            <title>Arun\'s Treasury Branches</title>
            <link rel="icon" href="/favicon.ico" />
        </Head>
        <h1>accounts page</h1>
        <Link href="/">
            <a>Back to home</a>
        </Link>

    </Layout>
)

export default AccountPage