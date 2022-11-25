import { useEffect } from "react";
import { Navigate, redirect, useSearchParams } from "react-router-dom";
import useApiClient from "../hooks/apiclient";
import useAuth from "../hooks/useAuth";

const Login = () => {
  const [searchParams, setSearchParams] = useSearchParams();

  const token = searchParams.get("token");
  const username = searchParams.get("username");
  const {get} = useApiClient();

  const { authenticated, 
    // uuid, 
    setToken, 
    // token:t,
    setUsername
   } = useAuth();

  useEffect(() => {
    if (token) {
      setToken(token);
    }
    if (username) {
      setUsername(username);
    }
    // if (authenticated) {
    //   return;
    // }

    // get cookie
    const getCookie = async () => {
      await get("/login_web");

    }
    getCookie();
  });

  // console.log("authenticated: ", authenticated, " uuid: ", uuid, " token: ", t);  

  return authenticated ? (
    // <Navigate replace to="/overview" />
    <div>redirect goes here</div>
  ) : (
    <>
      <h1>Login</h1>
      <div>{token}</div>
      {/* <div>{uuid}</div> */}
      <div>if you see this you are not authenticated</div>
      <div>todo: instructions go here</div>
    </>
  );
};

export default Login;
