'use client'

import { signIn, signOut } from "next-auth/react";
const Buttons = () => {

    return (
      <>
        <button style={{ marginRight: 10 }} onClick={() => signIn()}>
          Sign in
        </button>
        <button style={{ marginRight: 10 }} onClick={() => signOut()}>
          Sign Out
        </button>
      </>
    );
  };

  export default Buttons;