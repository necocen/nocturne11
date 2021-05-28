import React, { useRef, useState } from "react";

type Props = {
    title?: string;
    body?: string;
    id?: string;
};

export function Form(props: Props) {
    const formRef = useRef<HTMLFormElement>(null);
    const deleteFormRef = useRef<HTMLFormElement>(null);
    const [title, setTitle] = useState(props.title ?? "");
    const [body, setBody] = useState(props.body ?? "");

    const submit = () => {
        if (formRef.current?.reportValidity()) {
            formRef.current?.submit();
        }
    };
    const submitDelete = () => deleteFormRef.current?.submit();

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
