import { Navigate, redirect, useSearchParams } from "react-router-dom";
import useAuth from "../hooks/useAuth";

const Login = () => {
  const [searchParams, setSearchParams] = useSearchParams();

  const token = searchParams.get("token");

  const { authenticated } = useAuth();

  return (
    <>
      {authenticated ? <Navigate replace to="/overview"/> : (
        <>
          <h1>Login</h1>
          <div>{token}</div>
        </>
      )}
    </>
  );
};

export default Login;
