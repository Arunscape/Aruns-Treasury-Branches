const { v4: uuidv4 } = require("uuid");
import React from "react";

export const AuthContext = React.createContext();

export const client_id = "b65a4f90-a35f-4345-a755-fa4c05c7a8d9";

export const tenant_id = "01aee9c7-5d1f-409d-b90a-c21e44a429e5";

export const authority = `https://login.microsoftonline.com/${tenant_id}`;

export const redirect_uri = "http://localhost:3000/cb";

type Action = { type: "LOGIN" } | { type: "LOGOUT" };

export const AuthReducer = (state: any, action: Action) => {
  switch (action.type) {
    case "LOGOUT":
      localStorage.clear();
      return state;
    case "LOGIN":
      return state;
    default:
      return state;
  }
};
