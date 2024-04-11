import { useRef, useEffect } from "react";

export function LoginButton({ clientId }: { clientId: string }) {
    const formRef = useRef<HTMLFormElement>(null);
    const inputRef = useRef<HTMLInputElement>(null);
    const loginButtonDivRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        loadScript(() => {
            window.google.accounts.id.initialize({
                client_id: clientId,
                callback: (response) => {
                    inputRef.current?.setAttribute("value", response.credential);
                    formRef.current?.submit();
                },
            });
            if (loginButtonDivRef.current) {
                window.google.accounts.id.renderButton(loginButtonDivRef.current, {
                    type: "standard",
                    theme: "outline",
                });
            }
        });
    }, [clientId]);

    return (
        <>
            <div ref={loginButtonDivRef} />
            <form action="/login" method="POST" ref={formRef}>
                <input type="hidden" name="id_token" ref={inputRef} />
            </form>
        </>
    );
}

export function LogoutButton() {
    const signOut = () => {
        window.location.href = "/logout";
    };
    return (
        <button type="button" onClick={signOut}>
            logout
        </button>
    );
}

function loadScript(callback: () => void) {
    const existingScript = document.getElementById("gsi-script");
    if (!existingScript) {
        const script = document.createElement("script");
        script.id = "gsi-script";
        script.async = true;
        script.onload = callback;
        script.type = "text/javascript";
        script.src = "https://accounts.google.com/gsi/client";
        document.body.appendChild(script);
    } else {
        callback();
    }
}
