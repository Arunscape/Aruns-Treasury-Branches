import { useRouter } from 'next/router';
import React from 'react'
import Layout from '../components/layout'

const ATB404: React.FC = () => {
    const router = useRouter();

    return <Layout>
        <div className="flex flex-col items-center justify-center min-h-screen py-2">
            <h1>404 - Page Not Found</h1>
            <button
                // href="#"
                type="button"
                className="inline-flex items-center justify-center px-4 py-2 text-base font-medium leading-6 text-white whitespace-no-wrap bg-indigo-600 border border-transparent rounded-md shadow-sm hover:bg-indigo-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-600"
                onClick={() => router.back()}
            >
                Go back
            </button>
        </div>
    </Layout>
}

export default ATB404;