import { useRouteError } from "react-router-dom";
import Header from "./components/Header";

export default function ErrorPage() {
  const error: any = useRouteError();
  console.error(error);

  return (
    <>
    <Header/>
    <div id="error-page">
      <h1>Oops!</h1>
      <p>Sorry, an unexpected error has occurred.</p>
      <p>
        <i>{error.statusText || error.message}</i>
      </p>
    </div>
    </>
  );
}
