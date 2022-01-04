import Head from 'next/head'
import Link from 'next/link'
import Layout from '../components/layout'


import type { NextPage } from 'next'

const Home: NextPage = () => {
  return (
    <Layout>


      <main className="flex flex-col items-center justify-center w-full flex-1 px-20 text-center">
        <h1 className="text-6xl font-bold">
          Welcome to{' '} m
        </h1>
      </main>


    </Layout>
  )
}

export default Home