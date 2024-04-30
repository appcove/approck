import { Err, Ok, Result } from "@granite/mod.mjs";
import { round } from "lodash-es";
import BuxInputText from "./mod.mjs";
import "./currency.mcss";

// src/components/CurrencyInput.ts
class BuxInputCurrency extends BuxInputText<number> {
    /// return a valid value or throw a new Error
    parse(text: string): Result<number, string> {
        text = text.trim();

        // remove extra characters
        text = text.replace(/[^0-9.]/g, "");

        let value = parseFloat(text);
        if (isNaN(value)) {
            return Err("Decimal number required.");
        }

        if (value > Number.MAX_SAFE_INTEGER) {
            return Err("Number must not exceed " + Number.MAX_SAFE_INTEGER.toLocaleString() + ".");
        }

        if (value < Number.MIN_SAFE_INTEGER) {
            return Err("Number must not be less than " + Number.MIN_SAFE_INTEGER.toLocaleString() + ".");
        }

        // round to 2
        value = round(value, 2);

        return Ok(value);
    }

    format(value: number): string {
        // Convert to locale string with 2 decimal places
        return "$" + value.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 });
    }
}

window.customElements.define("bux-input-text-currency", BuxInputCurrency);
export default BuxInputCurrency;
