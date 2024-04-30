import { BuxElement } from "./mod.mjs";

export abstract class BuxElementGroup<ROW_TYPE extends object> extends BuxElement {
    public on_remove: () => void = () => {
        console.log("unhandled on_remove() for", this);
    };
    public on_change: (row: Partial<ROW_TYPE>) => void = (row) => {
        console.log("unhandled on_change(", row, ") for", this);
    };

    abstract validate(): void;
    abstract set value(row: Partial<ROW_TYPE>);
    abstract get value(): Partial<ROW_TYPE>;
}
