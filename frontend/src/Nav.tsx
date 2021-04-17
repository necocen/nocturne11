import React from "react";
import { Calendar } from "./Calendar";
import { Months } from "./Months";

export function Nav() {
    return (
        <div id="side-nav-content">
            <Calendar />
            <Months />
        </div>
    );
}
