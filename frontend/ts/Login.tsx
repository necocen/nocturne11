import React, { useRef } from "react";
import { GoogleLogin, useGoogleLogout, GoogleLoginResponseOffline } from "react-google-login";

export function LoginButton() {
    const formRef = useRef<HTMLFormElement>(null);
    const inputRef = useRef<HTMLInputElement>(null);
    return (
        <>
            <GoogleLogin
                clientId={import.meta.env.SNOWPACK_PUBLIC_GOOGLE_CLIENT_ID}
                icon={false}
                onSuccess={(response) => {
                    if (isGoogleLoginResponseOffline(response)) {
                        // Something wrong
                        console.warn(response);
                        return;
                    }
                    inputRef.current?.setAttribute("value", response.tokenId);
                    formRef.current?.submit();
                }}
            />
            <form action="/login" method="POST" ref={formRef}>
                <input type="hidden" name="id_token" ref={inputRef} />
            </form>
        </>
    );
}

export function LogoutButton() {
    const { signOut } = useGoogleLogout({
        clientId: import.meta.env.SNOWPACK_PUBLIC_GOOGLE_CLIENT_ID,
        onLogoutSuccess: () => {
            window.location.href = "/logout";
        },
        onFailure: () => {
            console.warn("Failed to log out from Google");
            window.location.href = "/logout";
        },
    });

    return <button onClick={signOut}>logout</button>;
}

function isGoogleLoginResponseOffline(arg: any): arg is GoogleLoginResponseOffline {
    return arg !== null && typeof arg === "object" && typeof arg.code === "string";
}
