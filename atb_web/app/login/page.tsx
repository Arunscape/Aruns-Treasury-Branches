import { getServerSession } from "next-auth";
import { useSession, signIn, signOut } from "next-auth/react";

import Btns from "@/components/Btns";
import { authOptions } from "@/lib/auth";
import { getToken } from "next-auth/jwt";



// export async function getServerSideProps(context: any) {
//   return {
//     props: {
//       session: await getServerSession(
//         context.req,
//         context.res,
//         authOptions
//       ),
//     },
//   }
// }



export default async function Component() {
  // const { data: session } = useSession()
  const session = await getServerSession(authOptions as any)
  
  console.log("session", session)

  return (
    <>
      <h1>Server Session</h1>
      <pre>{JSON.stringify(session, null, 4)}</pre>
      <Btns />
    </>
  );
}
