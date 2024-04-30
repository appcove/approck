import { Err, Ok, Result } from "@granite/mod.mjs";
import "./mod.mcss";

abstract class BuxInputText<T> extends HTMLElement {
    private $input: HTMLInputElement;
    public on_change?: (value: T) => void;

    constructor() {
        super();
        this.$input = document.createElement("input");
        this.$input.type = "text";
        let name = this.getAttribute("name");
        if (name !== null) {
            this.$input.name = name;
        }

        // feed initial value in through standard mechanism
        if (this.hasAttribute("value")) {
            this.attributeChangedCallback("value", "", this.getAttribute("value") || "");
        }
    }

    connectedCallback(): void {
        this.appendChild(this.$input);
        this.$input.addEventListener("input", this.event_on_input.bind(this));
        this.$input.addEventListener("change", this.event_on_change.bind(this));
    }

    disconnectedCallback(): void {
        this.$input.removeEventListener("input", this.event_on_input.bind(this));
        this.$input.removeEventListener("change", this.event_on_change.bind(this));
    }

    static get observedAttributes(): string[] {
        return ["value", "trim", "required"];
    }

    attributeChangedCallback(name: string, oldValue: string, newValue: string): void {
        switch (name) {
            case "value":
                this.text = newValue;
                break;
            case "required":
                break;
            case "trim":
                break;
        }
    }

    // clear custom messages while typing
    private event_on_input(): void {
        this.$input.setCustomValidity("");
    }

    // revalidate once the user has finished typing
    private event_on_change(): void {
        const valid = this.validate();

        if (valid && this.on_change) {
            const r = this.value_result;
            if (r.is_ok) {
                this.$input.value = this.format(r.value);
                this.on_change(r.value);
            }
        }
    }

    public validate(): boolean {
        const r = this.value_result;

        if (r.is_ok) {
            this.$input.setCustomValidity("");
            this.$input.value = this.format(r.value);
            this.$input.title = "";
            return true;
        } else {
            this.$input.setCustomValidity(r.error);
            this.$input.title = r.error;
            return false;
        }
    }

    abstract format(value: T): string;
    abstract parse(text: string): Result<T, string>;

    /// Calling .value_result will return a Result<T, string> which you can manually check .is_ok on
    public get value_result(): Result<T, string> {
        return this.parse(this.$input.value);
    }

    /// Calling .value will return either a valid T or undefined if T is not valid
    public get value(): T | undefined {
        const r = this.value_result;
        if (r.is_ok) {
            return r.value;
        } else {
            return undefined;
        }
    }

    /// Setting undefined has zero effect on the field.
    public set value(value: T | undefined) {
        if (value !== undefined) {
            this.$input.value = this.format(value);
        }
    }

    public get text(): string {
        return this.$input.value;
    }
    public set text(text: string | undefined) {
        if (text !== undefined) {
            this.$input.value = text;
            this.validate();
        }
    }

    get attr_required(): boolean {
        return this.hasAttribute("required");
    }

    get attr_trim(): boolean {
        return this.hasAttribute("trim");
    }
}

export default BuxInputText;
