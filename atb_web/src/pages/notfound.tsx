import { useNavigate, useRouteError } from "react-router-dom";

export default () => {
  const navigate = useNavigate();
  const error = useRouteError();

  return (
    <>
      <button onClick={() => navigate("/")}>Go back home</button>
      <p>
        <i>{error?.statusText || error?.message || "¯\_(ツ)_/¯"}</i>
      </p>
    </>
  );
};
