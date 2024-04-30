import { computePosition } from "@floating-ui/dom";
import { HS, make_element } from "@granite/util.mjs";

import "./mod.mcss";

export interface Opts {
    $trigger: HTMLElement;
    $content: HTMLElement;
    title: string;
    class_list?: string[];
    on_close?: () => void;
}

export class Floater {
    $trigger: HTMLElement;
    $element: HTMLElement;

    $title: HTMLElement;
    $title_span: HTMLSpanElement;

    $close: HTMLAnchorElement | null;
    on_close: (() => void) | undefined;

    $content: HTMLElement;

    // create a constructor
    constructor(opts: Opts) {
        this.$trigger = opts.$trigger;

        this.on_close = opts.on_close;

        this.$element = make_element(
            "bux-floater",
            `
            <x-title>
                <span>${HS(opts.title)}</span>
                <!-- close button goes here -->
            </x-title>
            <!-- x-content goes here -->
        `,
        );

        if (opts.class_list !== undefined) {
            opts.class_list.forEach((cls) => {
                this.$element.classList.add(cls);
            });
        }

        // Title elements
        this.$title = this.$element.querySelector("x-title") as HTMLElement;
        this.$title_span = this.$element.querySelector("x-title > span") as HTMLSpanElement;

        // validate and append $content
        if (opts.$content.tagName !== "X-CONTENT") {
            throw Error("Content element must be a <x-content> tag");
        }

        this.$content = opts.$content;
        this.$element.appendChild(this.$content);

        // Close only shows up if we passed a callback
        if (opts.on_close === undefined) {
            this.$close = null;
        } else {
            this.$close = document.createElement("a");
            this.$close.href = "#";
            this.$close.textContent = "close";
            this.$close.addEventListener("click", (e) => {
                e.preventDefault();
                this.close();
            });
            this.$title.appendChild(this.$close);
        }

        document.body.appendChild(this.$element);
    }

    // create a method
    show() {
        computePosition(this.$trigger, this.$element).then(({ x, y }) => {
            this.$element.style.left = `${x}px`;
            this.$element.style.top = `${y}px`;
            this.$element.style.visibility = "visible";
        });
    }

    hide() {
        this.$element.style.visibility = "hidden";
    }

    close() {
        document.body.removeChild(this.$element);
        if (this.on_close !== undefined) {
            this.on_close();
        }
    }
}
