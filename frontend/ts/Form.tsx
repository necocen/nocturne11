import React, { useRef, useState, useCallback } from "react";
import { useSessionStorage } from "./storage";

type Props = {
    title?: string;
    body?: string;
    id?: string;
};

export function Form(props: Props) {
    const formRef = useRef<HTMLFormElement>(null);
    const deleteFormRef = useRef<HTMLFormElement>(null);
    const [sessionStorageTitle, setSessionStorageTitle] = useSessionStorage<string | undefined>(`post-title${props.id ?? ""}`, undefined);
    const [sessionStorageBody, setSessionStorageBody] = useSessionStorage<string | undefined>(`post-body${props.id ?? ""}`, undefined);
    const [title, rawSetTitle] = useState(sessionStorageTitle ?? props.title ?? "");
    const [body, rawSetBody] = useState(sessionStorageBody ?? props.body ?? "");

    const setTitle = useCallback(
        (title: string) => {
            rawSetTitle(title);
            setSessionStorageTitle(title);
        },
        [rawSetTitle, setSessionStorageTitle]
    );
    const setBody = useCallback(
        (body: string) => {
            rawSetBody(body);
            setSessionStorageBody(body);
        },
        [rawSetBody, setSessionStorageBody]
    );

    const submit = useCallback(() => {
        if (formRef.current?.reportValidity()) {
            formRef.current?.submit();
            setSessionStorageTitle(undefined);
            setSessionStorageBody(undefined);
        }
    }, [formRef, setSessionStorageTitle, setSessionStorageBody]);
    const submitDelete = useCallback(() => {
        deleteFormRef.current?.submit();
        setSessionStorageTitle(undefined);
        setSessionStorageBody(undefined);
    }, [deleteFormRef, setSessionStorageTitle, setSessionStorageBody]);

    return (
        <>
            <form id={props.id ? "form-update" : "form-create"} method="POST" action={props.id ? "/admin/update" : "/admin/create"} ref={formRef}>
                <fieldset>
                    <h4>
                        <label htmlFor="post-form-title">タイトル</label>
                    </h4>
                    <p>
                        <input type="text" name="title" id="post-form-title" value={title} onChange={(e) => setTitle(e.target.value)} required />
                        <input type="text" id="dummy-to-prevent-submit" style={{ display: "none" }} />
                    </p>
                    <h4>
                        <label htmlFor="post-form-body">本文</label>
                    </h4>
                    <p>
                        <textarea name="body" id="post-form-body" value={body} onChange={(e) => setBody(e.target.value)} required />
                    </p>
                </fieldset>
                <p>
                    {props.id && <input type="hidden" name="id" id="post-form-id" value={props.id} />}
                    <button type="button" onClick={submit}>
                        送信
                    </button>
                </p>
            </form>
            {props.id && (
                <form id="form-delete" method="POST" action="/admin/delete" ref={deleteFormRef}>
                    <p>
                        <input type="hidden" name="id" id="post-form-id" value={props.id} />
                        <button type="button" onClick={submitDelete}>
                            削除
                        </button>
                    </p>
                </form>
            )}
        </>
    );
}
