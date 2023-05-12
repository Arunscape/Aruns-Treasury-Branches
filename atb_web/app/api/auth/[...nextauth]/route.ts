import NextAuth from "@auth/nextjs"

const handler = NextAuth({
  ...
})

export { handler as GET, handler as POST }