import { createContext, useContextProvider, useStore } from "@builder.io/qwik";

export interface AuthState {
    token: string;
}

export const AuthContext = createContext<AuthState>('auth');


