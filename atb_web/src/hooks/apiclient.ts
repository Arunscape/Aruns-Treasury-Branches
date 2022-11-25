import { useEffect, useState } from "react";
import useAuth from "./useAuth";

const useApiClient = () => {

    // will be using httponly cookiee
    const { token, 
        // uuid 
    } = useAuth();
    const backend = "http://localhost:8080";

    const post = async (path: string, body: any) => {
        await fetch(`${backend}${path}`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "Authorization": `Bearer ${token}`
            },
            body: JSON.stringify(body),
            credentials: "include",
        });
    };

    const get = async (path: string) => {
        await fetch(`${backend}${path}`, {
            method: "GET",
            headers: {
                // "Content-Type": "application/json",
                "Authorization": `Bearer ${token}`,
            },
            credentials: "include",
        });
    };


    return {
        get,
        post,
    }


}

export default useApiClient;