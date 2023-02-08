import { useEffect, useState } from "react"

export const useAuth = () => {

    const [token, setT] = useState<string | null>(localStorage.getItem("token"));

    const setToken = (t: string) => {
        if (t === token) {
            return;
        }
        localStorage.setItem("token", t);
        setT(t);
    }

    const logout = () => {
        setT(null);
        localStorage.removeItem("token");
    }

    // const refreshToken = async () => {
    //     if (!token) return;
        
    // }

    // useEffect(() => {
    //     const interval = setInterval(() => {
    //         refreshToken();
    //     }, 43200000); // 12 hours

    //     return () => clearInterval(interval);
    // }, [])

    return { token, setToken, logout }

}