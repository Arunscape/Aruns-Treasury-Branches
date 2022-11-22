import { useRouter } from "next/router";
import { useEffect } from "react";
import useAuth from "../hooks/useAuth";

const ProtectedPage = (props: any) => {
  const { authenticated } = useAuth();
  const router = useRouter();

  useEffect(() => {
    if (!authenticated) {
      router.push("/");
    }
  });

  return <>{authenticated && props.children}</>;
};

export default ProtectedPage;
