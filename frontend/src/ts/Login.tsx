import { useRef, useEffect } from "react";

export function LoginButton() {
    const formRef = useRef<HTMLFormElement>(null);
    const inputRef = useRef<HTMLInputElement>(null);
    const loginButtonDivRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        loadScript(() => {
            (window as any).google.accounts.id.initialize({
                client_id: import.meta.env.VITE_PUBLIC_GOOGLE_CLIENT_ID,
                callback: (response: any) => {
                    inputRef.current?.setAttribute("value", response.credential);
                    formRef.current?.submit();
                },
            });
            (window as any).google.accounts.id.renderButton(loginButtonDivRef.current!, {
                theme: "outline",
            });
        });
    }, []);

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
    return <button onClick={signOut}>logout</button>;
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
