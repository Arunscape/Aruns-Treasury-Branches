import { AppProps } from 'next/app';
import { MantineProvider } from '@mantine/core';
import Layout from '../components/layout';


// SuperTokens.init({
//   appInfo: {
//       appName: "Arun's Treasury Branches",
//       apiDomain: "http://localhost:8000",
//       websiteDomain: "http://localhost:3000",
//       apiBasePath: "/auth",
//       websiteBasePath: "/"
//   },
//   recipeList: [
//       Passwordless.init({
//           contactMethod: "EMAIL_OR_PHONE"
//       }),
//       Session.init()
//   ]
// });

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