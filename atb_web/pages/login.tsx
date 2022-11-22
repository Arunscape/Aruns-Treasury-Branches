import Link from "next/link";
import { useRouter } from "next/router";
import { useEffect } from "react";
import useAuth from "../hooks/useAuth";


const Login = () => {
    const {setToken} = useAuth();

    const router = useRouter();

    const {token} = router.query;

    useEffect(() => {
        if (token) {
            setToken(token as string);
            router.push("/deposits");
        }
    })

    return <>
    <div>Logging in...</div>
    <div>If you see this for a long time then your token isn't valid</div>
    <Link href="/">
        Go back home
    </Link>

    </>
}


export default Login;
