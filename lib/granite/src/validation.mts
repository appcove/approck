import { Err, Ok, Result } from "@crate/mod.mjs";

export function default_validator<T>(v: T | undefined): Result<T, string> {
    if (v === undefined) {
        return Err("Required");
    }
    return Ok(v);
}

export function default_validator_nullable<T>(v: T | null | undefined): Result<T | null, string> {
    if (v === undefined) {
        return Ok(null);
    }
    return Ok(v);
}

export function make_string_validator(
    opts: { max_length?: number; required?: boolean; allow_undefined?: boolean },
): (value: string | undefined) => Result<string, string> {
    return (value: string | undefined) => {
        if (value === undefined) {
            if (opts.required) {
                return Err("Required");
            } else {
                value = "";
            }
        }

        value = value.trim();

        if (opts.required !== undefined) {
            if (value.length === 0) {
                return Err("Required");
            }
        }

        if (opts.max_length !== undefined) {
            if (value.length > opts.max_length) {
                return Err("Too long");
            }
        }

        return Ok(value);
    };
}

export function make_number_validator(
    opts: { required?: boolean; min?: number; max?: number },
): (value: number | undefined) => Result<number, string> {
    return (value: number | undefined) => {
        if (value === undefined) {
            if (opts.required) {
                return Err("Required");
            } else {
                value = 0;
            }
        }

        if (opts.min !== undefined) {
            if (value < opts.min) {
                return Err("Too small");
            }
        }

        if (opts.max !== undefined) {
            if (value > opts.max) {
                return Err("Too big");
            }
        }

        return Ok(value);
    };
}
