import type { NextPage } from 'next'
import Head from 'next/head'
import Header from './Header'

const Layout: NextPage = props => <>

  <Head>
    <title>Arun&apos;s Treasury Branches</title>
    <link rel="icon" href="/favicon.ico" />
  </Head>
  <Header user={{ name: 'Arunscape', image: "https://avatars.githubusercontent.com/arunscape" }} tabs={['Home', 'Account', 'FAQ']} />
  {props.children}

</>

export default Layout