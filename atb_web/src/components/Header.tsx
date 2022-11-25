import React from "react";
import { Link } from "react-router-dom";
import useApiClient from "../hooks/apiclient";
import useAuth from "../hooks/useAuth";

const Header = () => {

  const {username} = useAuth();
  return (
    <header>
      <h1>Arun's Treasury Branches</h1>
      <nav>
        <ul>
          <li>
            <Link to="/deposit">Deposit</Link>
          </li>
        </ul>
        <ul>
          <li>
            <Link to="/trade">Trade</Link>
          </li>
        </ul>
        <ul>
          <li>
            <Link to="/login">Login</Link>
          </li>
        </ul>
      </nav>
      <div>Welcome, {username}</div>
    </header>
  );
};

export default Header;
