import React, { useState, useContext, useEffect } from "react"
// import jwt_decode, {JwtPayload} from "jwt-decode";
import { useLocalStorage } from '@mantine/hooks';


// const checkJWT = (token: string) => {
//   const payload: JwtPayload = jwt_decode(token);
//   return payload?.sub;
// };


const useAuth = () => {

  const [token, setToken] = useLocalStorage<string | undefined>({ key: 'token', defaultValue: undefined });
  const [username, setUsername] = useLocalStorage<string | undefined>({ key: 'username', defaultValue: undefined });
  const [authenticated, setAuthenticated] = useState(false);
  // const [uuid, setUuid] = useState<string | null>(null);


  useEffect(() => {
    if (!token) {
      return;
    };
    // const uuid = checkJWT(token);
    // if (!uuid) {
    //   return;
    // };
    // setUuid(uuid);
    setAuthenticated(true);
  }, [token, authenticated,
    // uuid,
  ]);




  return {
    authenticated,
    token,
    // uuid,
    username,
    setUsername,
    setToken,
  }

}

export default useAuth;
