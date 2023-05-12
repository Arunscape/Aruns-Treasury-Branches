import { getServerSession } from "next-auth";
import { useSession, signIn, signOut } from "next-auth/react";
import { authOptions } from "../api/auth/[...nextauth]/route";
import Btns from "@/components/Btns";



export default async function Component() {
  const session = await getServerSession(authOptions as any);

  return (
    <>
      <h1>Server Session</h1>
      <pre>{JSON.stringify(session)}</pre>
      <Btns />
    </>
  );
}
