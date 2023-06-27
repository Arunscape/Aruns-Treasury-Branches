import { getToken } from "next-auth/jwt"
import { NextResponse } from "next/server"

const handler = async (req, res) => {

    const token = await getToken({ req })
    console.log(token)

    if (!token?.accessToken) {
        return new Response("No access token", {
            status: 401,
        })
    }

    const { accessToken } = token;

    console.log("step 1 we have an access token: ", accessToken)

    // POST https://user.auth.xboxlive.com/user/authenticate
    // {
    //    "Properties": {
    //        "AuthMethod": "RPS",
    //        "SiteName": "user.auth.xboxlive.com",
    //        "RpsTicket": "d=<access token>" // your access token from the previous step here
    //    },
    //    "RelyingParty": "http://auth.xboxlive.com",
    //    "TokenType": "JWT"
    // }

    const xblToken = await fetch("https://user.auth.xboxlive.com/user/authenticate", {  
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            "Accept": "application/json",
        },
        body: JSON.stringify({
            Properties: {
                AuthMethod: "RPS",
                SiteName: "user.auth.xboxlive.com",
                RpsTicket: `d=${accessToken}`,
            },
            RelyingParty: "http://auth.xboxlive.com",
            TokenType: "JWT",
        }),
    }).then((res) => res.json())


    console.log("step 2 we have an xbl token: ", xblToken)

    return NextResponse.json(xblToken)

}


export { handler as GET }