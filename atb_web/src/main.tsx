import React, { Suspense } from 'react'
import ReactDOM from 'react-dom/client'
import {
  createBrowserRouter,
  Navigate,
  RouterProvider,
} from "react-router-dom";

import "./index.css"

import Loading from "./components/loading";
import NotFound from "./pages/notfound";
import Login from './pages/login';
import Root from './components/root';
import Summary from './pages/summary';
import { useAuth } from './hooks/useAuth';
import Home from './pages/home';

// const Home = React.lazy(() => import("./pages/home"));

const router = createBrowserRouter([
  {
    path: "/",
    element: <Root/>,
    errorElement: <NotFound/>,
    children: [
      {
        path: "/",
        element: <Home/>,
      },
      {
        path: "login",
        element: <Login/>,
      },
      {
        path: "summary",
        element: <Suspense fallback={<Loading/>}><Summary/></Suspense>,
      }
    ],
  },
]);

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router}/>
  </React.StrictMode>,
)
