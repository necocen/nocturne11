import React, { useCallback } from "react";

export function SearchButton() {
    const searchForm = document.getElementById("search-box");
    const toggle = useCallback(() => {
        if (!searchForm) {
            return;
        }
        if (searchForm.classList.contains("search-box-hidden")) {
            for (const input of searchForm.getElementsByTagName("input")) {
                input.disabled = false;
            }
            for (const button of searchForm.getElementsByTagName("button")) {
                button.disabled = false;
            }
            searchForm.classList.remove("search-box-hidden");
        } else {
            for (const input of searchForm.getElementsByTagName("input")) {
                input.disabled = true;
            }
            for (const button of searchForm.getElementsByTagName("button")) {
                button.disabled = true;
            }
            searchForm.classList.add("search-box-hidden");
        }
    }, [searchForm]);

    return <button onClick={toggle}>search</button>;
}
