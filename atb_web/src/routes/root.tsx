import { Outlet } from "react-router-dom";
import Header from "../components/Header";

const RootRoute = () => {
  return (
    <>
      <Header />
      <Outlet />
    </>
  );
};

export default RootRoute;
