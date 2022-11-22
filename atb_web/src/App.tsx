import React from "react";
import { createBrowserRouter, RouterProvider, Route } from "react-router-dom";

import "./App.css";
import Header from "./components/Header";
import ErrorPage from "./error-page";
import Deposit from "./routes/deposit";
import Home from "./routes/Home";
import Login from "./routes/login";
import Overview from "./routes/overview";
import Root from "./routes/root";
const router = createBrowserRouter([
  {
    path: "/",
    element: <Root />,
    errorElement: <ErrorPage />,
    children: [
      {
        path: "/",
        element: <Home />,
      },
      {
        path: "/overview",
        element: <Overview />,
      },
      {
        path: "/deposit",
        element: <Deposit />,
      },
      {
        path: "/login",
        element: <Login/>,
      }
    ],
  },
]);
function App() {
  return (
    <>
      <RouterProvider router={router} />
    </>
  );
}

export default App;
