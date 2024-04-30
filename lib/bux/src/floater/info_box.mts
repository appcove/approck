import { HS, make_element } from "@granite/util.mjs";
import { Floater, Opts as ParentOpts } from "./mod.mjs";

import "./info_box.mcss";

export interface Opts {
    html?: string;
    text?: string;
    title?: string;
}

export class InfoBox extends Floater {
    constructor(opts: Omit<ParentOpts, "$content" | "title"> & Opts) {
        const $content = document.createElement("x-content");

        if (opts.html !== undefined) {
            $content.innerHTML = opts.html;
        }

        if (opts.text !== undefined) {
            $content.textContent = opts.text;
        }

        const class_list = opts.class_list || [];
        class_list.push("InfoBox");

        // default on_close to an empty function so that the `close` button shows up
        // and will have default behavior
        let on_close = opts.on_close || (() => {});

        // Check if the title is provided, otherwise default it to "Search"
        const super_opts = {
            ...opts,
            class_list,
            $content,
            on_close,
            title: opts.title || "Information",
        };

        super(super_opts);
    }
}
