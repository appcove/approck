export class BuxToolbar extends HTMLElement {
    protected $shadow: ShadowRoot;
    private $undo?: HTMLButtonElement;
    private $redo?: HTMLButtonElement;
    private $clear?: HTMLButtonElement;

    on_undo?: () => void;
    on_redo?: () => void;
    on_clear?: () => void;

    constructor() {
        super();
        // create a shadow root
        this.$shadow = this.attachShadow({ mode: "open" });

        let $style = document.createElement("style");
        $style.textContent = `
            button {
                display: inline-block;
                margin: 1pt;
                font-size: 14pt;
                padding: 0 2pt;
            }
        `;
        this.$shadow.appendChild($style);

        if (this.hasAttribute("undo")) {
            this.add_undo(() => {});
        }

        if (this.hasAttribute("redo")) {
            this.add_redo(() => {});
        }

        if (this.hasAttribute("clear")) {
            this.add_clear(() => {});
        }
    }

    connectedCallback() {
        if (this.$undo) {
            this.$undo.addEventListener("click", this.on_undo_click.bind(this));
        }

        if (this.$redo) {
            this.$redo.addEventListener("click", this.on_redo_click.bind(this));
        }

        if (this.$clear) {
            this.$clear.addEventListener("click", this.on_clear_click.bind(this));
        }
    }

    disconnectedCallback() {
        if (this.$undo) {
            this.$undo.removeEventListener("click", this.on_undo_click.bind(this));
        }

        if (this.$redo) {
            this.$redo.removeEventListener("click", this.on_redo_click.bind(this));
        }

        if (this.$clear) {
            this.$clear.removeEventListener("click", this.on_clear_click.bind(this));
        }
    }

    add_undo(cb: () => void) {
        if (!this.$undo) {
            this.$undo = document.createElement("button");
            this.$undo.type = "button";
            this.$undo.title = "Undo";
            this.$undo.innerText = "⟲";
            this.$shadow.appendChild(this.$undo);
        }
        this.on_undo = cb;
    }

    add_redo(cb: () => void) {
        if (!this.$redo) {
            this.$redo = document.createElement("button");
            this.$redo.type = "button";
            this.$redo.title = "Redo";
            this.$redo.innerText = "⟳";
            this.$shadow.appendChild(this.$redo);
        }
        this.on_redo = cb;
    }

    add_clear(cb: () => void) {
        if (!this.$clear) {
            this.$clear = document.createElement("button");
            this.$clear.type = "button";
            this.$clear.title = "Clear";
            this.$clear.innerText = "⊘";
            this.$shadow.appendChild(this.$clear);
        }
        this.on_clear = cb;
    }

    private on_undo_click(evt: Event) {
        evt.preventDefault();
        evt.stopPropagation();
        this.on_undo && this.on_undo();
    }

    private on_redo_click(evt: Event) {
        evt.preventDefault();
        evt.stopPropagation();
        this.on_redo && this.on_redo();
    }

    private on_clear_click(evt: Event) {
        evt.preventDefault();
        evt.stopPropagation();
        this.on_clear && this.on_clear();
    }
}

customElements.define("bux-toolbar", BuxToolbar);
