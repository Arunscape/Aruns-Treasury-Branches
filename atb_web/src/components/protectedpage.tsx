import { useAuth } from "../hooks/useAuth";
import { Navigate, useLocation } from "react-router-dom";

export default ({children}: {children: JSX.Element}) => {
    const {token} = useAuth();
    const location = useLocation();

    if (!token) {
        return <Navigate to="/" state={{ from: location }} replace/>
    }

    return children;
}