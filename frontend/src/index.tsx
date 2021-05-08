import React from "react";
import ReactDOM from "react-dom";
import { Nav } from "./Nav";

ReactDOM.render(<Nav />, document.getElementById("side-nav"));

function appendClassName(span: HTMLSpanElement, className: string) {
    const classNames = span.className.split(" ").filter((c) => c.length > 0);
    classNames.push(className);
    span.className = classNames.join(" ");
}

function modifyYakumonos() {
    for (const paragraph of document.querySelectorAll("article p")) {
        const modified = [...paragraph.childNodes].flatMap((node) => {
            if (node.nodeType == node.TEXT_NODE) {
                return [...(node.textContent ?? "")]
                    .map((c) => {
                        // 約物を検出して<span>で囲みます
                        if ("「『（〈【｛［《〔“".includes(c)) {
                            const span = document.createElement("span");
                            appendClassName(span, "yakumono-open-bracket");
                            span.appendChild(document.createTextNode(c));
                            return span;
                        } else if ("」』）〉】｝］》〕”".includes(c)) {
                            const span = document.createElement("span");
                            appendClassName(span, "yakumono-close-bracket");
                            span.appendChild(document.createTextNode(c));
                            return span;
                        } else if ("、。，．".includes(c)) {
                            const span = document.createElement("span");
                            appendClassName(span, "yakumono-punctuation");
                            span.appendChild(document.createTextNode(c));
                            return span;
                        } else if ("・".includes(c)) {
                            const span = document.createElement("span");
                            appendClassName(span, "yakumono-interpunct");
                            span.appendChild(document.createTextNode(c));
                            return span;
                        } else if ("／＼！？".includes(c)) {
                            const span = document.createElement("span");
                            appendClassName(span, "yakumono-other");
                            span.appendChild(document.createTextNode(c));
                            return span;
                        } else {
                            return document.createTextNode(c);
                        }
                    })
                    .reduce((acc, r) => {
                        // 連続するTextノードを連結
                        if (acc.length == 0) {
                            return [r];
                        }
                        const last = acc.pop()!;
                        if (last.nodeType == node.TEXT_NODE && r.nodeType == node.TEXT_NODE) {
                            last.textContent = (last.textContent ?? "") + (r.textContent ?? "");
                            acc.push(last);
                        } else {
                            acc.push(last);
                            acc.push(r);
                        }
                        return acc;
                    }, [] as (HTMLSpanElement | Text)[]).map((node) => {
                        if (node.nodeType == node.TEXT_NODE) {
                            const span = document.createElement("span");
                            span.appendChild(document.createTextNode(node.textContent ?? ""));
                            appendClassName(span, "non-yakumono");
                            return span;
                        } else {
                            return node;
                        }
                    })
            } else {
                return [node];
            }
        });

        paragraph.childNodes.forEach((node) => node.remove());
        modified.forEach((node) => paragraph.append(node));
    }
}
modifyYakumonos();
