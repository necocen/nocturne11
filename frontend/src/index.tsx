import React from "react";
import ReactDOM from "react-dom";
import { Nav } from "./Nav";

ReactDOM.render(<Nav />, document.getElementById("side-nav"));

function modifyYakumonos() {
    for (const paragraph of document.querySelectorAll("article p")) {
        const modified = [...paragraph.childNodes].flatMap((node) => {
            if (node.nodeType == node.TEXT_NODE) {
                return [...(node.textContent ?? "")]
                    .map((c) => {
                        // 約物を検出して<span>で囲みます
                        if (c == "「") {
                            const span = document.createElement("span");
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
                        if (acc[acc.length - 1].nodeType == node.TEXT_NODE && r.nodeType == node.TEXT_NODE) {
                            const last = acc.pop()!;
                            last.textContent = (last.textContent ?? "") + (r.textContent ?? "");
                            acc.push(last);
                        } else {
                            acc.push(r);
                        }
                        return acc;
                    }, [] as (HTMLSpanElement | Text)[]);
            } else {
                return [node];
            }
        });

        paragraph.childNodes.forEach((node) => node.remove());
        modified.forEach((node) => paragraph.append(node));
    }
}

modifyYakumonos();
