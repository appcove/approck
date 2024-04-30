export type Result<T, E> = { is_ok: true; value: T } | { is_ok: false; error: E };

// Helper functions to create a Result
export function Ok<T, E>(value: T): Result<T, E> {
    return { is_ok: true, value };
}

export function Err<T, E>(error: E): Result<T, E> {
    return { is_ok: false, error };
}

/// a function which validates a value, optionally modifing it, and returning a result or error
export type ValueValidator<VALUE> = (value: VALUE | undefined) => Result<VALUE, string>;

/// this converts a row type to a type where each key is a result
export type StructResult<STRUCT extends object> = {
    [P in keyof STRUCT]: Result<STRUCT[P], string>;
};

// Define a generic validator type for any type T where each validator is optional
export type StructValidator<STRUCT extends object> = {
    [P in keyof STRUCT]: ValueValidator<STRUCT[P]>;
};

export type Struct$id<STRUCT extends object> = {
    $id: string;
} & STRUCT;
