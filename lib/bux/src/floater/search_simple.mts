import { HS, make_element } from "@granite/util.mjs";
import { Floater, Opts as ParentOpts } from "./mod.mjs";

import "./search_simple.mcss";

export interface Opts {
    search_button_label?: string;
    title?: string;
}

export class SearchSimple extends Floater {
    protected $button: HTMLButtonElement;
    protected $result: HTMLElement;

    constructor(opts: Omit<ParentOpts, "$content" | "title"> & Opts) {
        const $content = make_element(
            "x-content",
            `
            <x-search>
                <input type="text" placeholder="Search...">
                <button>${HS(opts.search_button_label || "Search")}</button>
            </x-search>
            <x-result>
            </x-result>
        `,
        );

        const class_list = opts.class_list || [];
        class_list.push("SearchSimple");

        // Check if the title is provided, otherwise default it to "Search"
        const super_opts = {
            ...opts,
            class_list,
            $content,
            title: opts.title || "Search",
        };

        super(super_opts);

        this.$button = $content.querySelector("x-search > button") as HTMLButtonElement;
        this.$result = $content.querySelector("x-result") as HTMLElement;

        // add event handler to button and add a content to results
        this.$button.addEventListener("click", () => {
            this.$result.innerHTML =
                `<table class="table table-lined"><tr><td>Result 1</td><td>Result 2</td></tr></table>`;
            this.$result.style.display = "block";
        });
    }
}
