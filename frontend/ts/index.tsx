import React from "react";
import ReactDOM from "react-dom";
import { Nav } from "./Nav";
import { LoginButton, LogoutButton } from "./Login";

ReactDOM.render(<Nav />, document.getElementById("side-nav"));
const button = document.getElementById("google-login");
if (button) {
    ReactDOM.render(<LoginButton />, button);
}

const logout = document.getElementsByClassName("logout-button");
for (const el of logout) {
    ReactDOM.render(<LogoutButton />, el);
}

// rel="external"のついたリンクは別タブで開く
function modifyExternalLinks() {
    for (const link of document.getElementsByTagName("a")) {
        if (link.rel.split(" ").includes("external")) {
            link.onclick = () => {
                window.open(link.href);
                return false;
            };
        }
    }
}
modifyExternalLinks();

if (window.addEventListener) {
    window.addEventListener('AutoPagerize_DOMNodeInserted', modifyExternalLinks, false);
    window.addEventListener('AutoPatchWork.DOMNodeInserted', modifyExternalLinks, false);
}
