import NextAuth from "next-auth"
import AzureADProvider from "next-auth/providers/azure-ad";

if (!process.env.AZURE_AD_CLIENT_ID) throw new Error("AZURE_AD_CLIENT_ID is not defined")

const handler = NextAuth({
  providers: [
    AzureADProvider({
      // clientId: process.env.AZURE_AD_CLIENT_ID,
      clientSecret: process.env.AZURE_AD_CLIENT_SECRET || "AZURE_AD_CLIENT_SECRET is not defined",
      // tenantId: process.env.AZURE_AD_TENANT_ID,
      clientId: "b65a4f90-a35f-4345-a755-fa4c05c7a8d9",
      tenantId: "01aee9c7-5d1f-409d-b90a-c21e44a429e5",
    }),
  ]
})

export { handler as GET, handler as POST }