import useAuth from "../hooks/useAuth";
import { Navigate, redirect } from "react-router-dom";

const ProtectedPage = (props: any) => {
  const {authenticated } = useAuth();

  return authenticated ? <>{props.children}</> : <Navigate replace to="/"/>;
};

export default ProtectedPage;
