import React, { useCallback } from "react";

export function SearchButton() {
    const searchForm = document.getElementById("search-box");
    const toggle = useCallback(() => {
        if (!searchForm) {
            return;
        }
        if (searchForm.classList.contains("search-box-hidden")) {
            searchForm.classList.remove("search-box-hidden");
        } else {
            searchForm.classList.add("search-box-hidden");
        }
    }, [searchForm]);

    return <button onClick={toggle}>search</button>;
}
