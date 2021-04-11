import React from "react";

export function Nav() {
    const location = window.location.pathname;
    return (<b>{location}</b>);
}
