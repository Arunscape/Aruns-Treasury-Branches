import Provider from 'next-auth/providers'
export const McAuthProvider: Provider.OAuth = {
    id: "mc-auth",
    name: "mc-auth",
    type: "oauth",
    version: "2.0",
    //params:	Extra URL params sent when calling accessTokenUrl
    authorizationUrl: "https://mc-auth.com/oAuth2/authorize",
    tokenUrl: "https://mc-auth.com/oAuth2/token",

    clientId: process.env.MCAUTH_CLIENT_ID,
    clientSecret: process.env.MCAUTH_CLIENT_SECRET,
    redirect_uri: "http://localhost:3000",
    response_type: "code",
    scope: "profile",
    protection: "state",
    headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
        UserAgent: "NextAuth"
    },

    async profile(profile, tokens) {
      // You can use the tokens, in case you want to fetch more profile information
      // For example several OAuth providers do not return email by default.
      // Depending on your provider, will have tokens like `access_token`, `id_token` and or `refresh_token`
      return {
        profile,
        tokens
      }
    },
  })

//   {
//     "access_token": "<The access_token>",
//     "token_type": "Bearer",
//     "expires_in": 3600,
//     "scope": "profile",
//     "state": "<The same value provided for `state` in step 2>",
//     "data": {
//       "uuid": "407b28ede7bd451693d93361fecb7889",
//       "profile": {
//         "id": "407b28ede7bd451693d93361fecb7889",
//         "name": "Sprax2013",
//         "properties": [
//           {
//             "name": "textures",
//             "value": "<Base64 string>",
//             "signature": "<Base64 string; signed data using Yggdrasil's private key>"
//           }
//         ]
//       }
//     }
//   }