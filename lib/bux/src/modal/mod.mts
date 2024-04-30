import { HS } from "@granite/util.mjs";

import "./mod.mcss";

export interface Opts {
    $content: HTMLElement;
    title: string;
    class_list?: string[];
    on_close?: () => void;
}

export class BuxModal {
    $dialog: HTMLDialogElement;
    $main: HTMLElement;
    $close: HTMLButtonElement;

    // create a constructor
    constructor(opts: Opts) {
        this.$dialog = document.createElement("dialog");
        this.$dialog.classList.add("bux-modal");
        this.$dialog.innerHTML = `
            <header>
                <span>${HS(opts.title)}</span>
            </header>
            <main></main>
            <footer>
                <button type="button" name="close">Close</button>
            </footer>
        `;

        this.$main = this.$dialog.querySelector("main")!;
        this.$close = this.$dialog.querySelector("footer > button[name=close]")!;

        this.$main.appendChild(opts.$content);

        if (opts.class_list) {
            this.$dialog.classList.add(...opts.class_list);
        }

        document.body.appendChild(this.$dialog);

        this.$close.addEventListener("click", () => {
            this.remove();
        });
    }

    hide() {
        this.$dialog.close();
    }

    show() {
        this.$dialog.showModal();
    }

    remove() {
        this.$dialog.remove();
    }
}
