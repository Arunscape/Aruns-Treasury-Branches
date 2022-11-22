'use client';

import React, { useState, useContext, useEffect } from "react"
import { SignJWT, jwtVerify, importSPKI, importPKCS8, decodeJwt } from 'jose';
import { useLocalStorage } from '@mantine/hooks';


const checkJWT = (token: string | null) => true;

const getUuid = (token: string | null) => {

  if (!token) return null;

  try {
    const { payload } = decodeJwt(token);
  
    // @ts-ignore - uuid is a string
    return payload?.uuid;
  } catch(e) {
    return null;
  }

}

const useAuth = () => {

  const [token, setToken] = useLocalStorage<string | null>({key: 'token', defaultValue: null});

  const authenticated = checkJWT(token);


  if (!authenticated) {
    setToken(null);
  }


  const uuid = getUuid(token);

  return {
    authenticated,
    token,
    uuid,
    // username,
    setToken,
  }

}

export default useAuth;

