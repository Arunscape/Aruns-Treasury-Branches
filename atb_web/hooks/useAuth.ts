
import React, { useState, useContext, useEffect } from "react"
import { SignJWT, jwtVerify, importSPKI, importPKCS8, decodeJwt } from 'jose';
import { useLocalStorage } from '@mantine/hooks';


const checkJWT = (token: string | null) => false;

const getUuid = (token: string | null) => {

  if (!token) return null;

  const { payload } = decodeJwt(token);

  // @ts-ignore - uuid is a string
  return payload?.uuid;

}

const useAuth = () => {

  const [token, setToken] = useLocalStorage<string | null>({key: 'token', defaultValue: null});
  const [username, setUsername] = useLocalStorage<string | null>({key: 'username', defaultValue: null});
  
  const uuid = getUuid(token);
  const authenticated = checkJWT(token);

  return {
    authenticated,
    token,
    uuid,
    username,
    setToken,
  }

}

export default useAuth;

