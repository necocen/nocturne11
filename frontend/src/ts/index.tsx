import { createRoot } from "react-dom/client";
import { Nav } from "./Nav";
import { LoginButton, LogoutButton } from "./Login";
import { Form } from "./Form";
import { SearchButton } from "./SearchButton";

const nav = document.getElementById("side-nav");
if (nav) {
    createRoot(nav).render(<Nav />);
}

const button = document.getElementById("google-login");
if (button) {
    createRoot(button).render(<LoginButton />);
}

const logout = document.getElementsByClassName("logout-button");
for (const el of logout) {
    createRoot(el).render(<LogoutButton />);
}

const diaryForm = document.getElementById("diary-form-slot");
if (diaryForm) {
    createRoot(diaryForm).render(<Form id={diaryForm.dataset.id} title={diaryForm.dataset.title} body={diaryForm.dataset.body} />);
}

const search = document.getElementById("search-button");
if (search) {
    createRoot(search).render(<SearchButton />);
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
    window.addEventListener("AutoPagerize_DOMNodeInserted", modifyExternalLinks, false);
    window.addEventListener("AutoPatchWork.DOMNodeInserted", modifyExternalLinks, false);
}
