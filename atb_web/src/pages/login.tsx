import { useEffect } from "react";
import { Navigate, useLocation, useNavigate, useSearchParams } from "react-router-dom";
import { useAuth } from "../hooks/useAuth";


const Login = () => {
    const location = useLocation();
    const {token, setToken} = useAuth();
    const [searchParams] = useSearchParams();

    const from = location.state?.from?.pathname || "/";

    const tokenFromUrl = searchParams.get("token");

    if (tokenFromUrl) {
        setToken(tokenFromUrl);
    }

    if (token) {
        return <Navigate to="/" replace/>;
    }

    return <>
    <h1>{"Login"}</h1>
    <h2>TODO</h2>
    <p>{"Here are some instructions to get a token (todo)"}</p>

    </>
}

export default Login