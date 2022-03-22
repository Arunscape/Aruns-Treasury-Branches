import { AppProps } from 'next/app';
import { MantineProvider } from '@mantine/core';
import Layout from '../components/layout';

export default function App(props: AppProps) {
  const { Component, pageProps } = props;

  return (
    <Layout>
      <MantineProvider
        withGlobalStyles
        withNormalizeCSS
        theme={{
          /** Put your mantine theme override here */
          colorScheme: 'dark',
        }}
      >
        <Component {...pageProps} />
      </MantineProvider>
    </Layout>
  );
}