import React from "react";
import ReactDOM from "react-dom";
import { Nav } from "./Nav";
import { LoginButton } from "./Login";

ReactDOM.render(<Nav />, document.getElementById("side-nav"));
const button = document.getElementById("google-login");
if (button) {
    ReactDOM.render(<LoginButton />, button);
}
