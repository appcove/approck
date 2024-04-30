import { Err, Ok, Result } from "@granite/mod.mjs";
import BuxInputText from "./mod.mjs";
import "./integer.mcss";

class BuxInputTextInteger extends BuxInputText<number> {
    /// return a valid value or throw a new Error
    parse(text: string): Result<number, string> {
        text = text.trim();

        // remove extra characters
        text = text.replace(/[^0-9.]/g, "");

        let intvalue = parseInt(text);
        if (isNaN(intvalue)) {
            return Err("Whole number required.");
        }

        if (intvalue > Number.MAX_SAFE_INTEGER) {
            return Err("Number must not exceed " + Number.MAX_SAFE_INTEGER.toLocaleString() + ".");
        }

        if (intvalue < Number.MIN_SAFE_INTEGER) {
            return Err("Number must not be less than " + Number.MIN_SAFE_INTEGER.toLocaleString() + ".");
        }

        return Ok(intvalue);
    }

    format(value: number): string {
        // convert to a comma separated string using browser localle
        return value.toLocaleString();
    }
}

window.customElements.define("bux-input-text-integer", BuxInputTextInteger);
export default BuxInputTextInteger;
