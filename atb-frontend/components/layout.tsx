import React from 'react'
import Head from 'next/head'
const Layout: React.FunctionComponent = props =>
  <div id="layout" className="flex flex-col items-center justify-center min-h-screen py-2">
    {/*language=PostCSS*/}
    <style jsx global>{`
        #layout {
            background-color: darkolivegreen;
        }
    `}
    </style>
    <Head>
      <title>Arun\'s Treasury Branches</title>
      <link rel="icon" href="/favicon.ico" />
    </Head>
    {/* <Header/> */}
    {props.children}

  </div>


export default Layout