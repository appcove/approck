import { Err, Ok, Result } from "@granite/mod.mjs";
import BuxInputText from "./mod.mjs";
import "./string.mcss";

class BuxInputTextString extends BuxInputText<string> {
    /// return a valid value or throw a new Error
    parse(value: string): Result<string, string> {
        if (this.attr_trim) {
            value = value.trim();
        }

        if (this.attr_required && value === "") {
            return Err("Required");
        }

        return Ok(value);
    }

    format(value: string): string {
        return value;
    }
}

window.customElements.define("bux-input-text-string", BuxInputTextString);
export default BuxInputTextString;
