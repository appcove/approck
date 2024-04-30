import { Err, Ok, Result, Struct$id, StructResult, StructValidator } from "@granite/mod.mjs";
import { v4 as uuidv4 } from "uuid";
import { BuxStepper } from "./mod.mjs";

type BuxStepperStructError<STRUCT> = {
    $id: string;
    sequence: number;
    key: keyof STRUCT;
    error: string;
};

type BuxStepperStructSequenceLog<STRUCT> = {
    prepend?: Struct$id<Partial<STRUCT>>; // add row to top, return ids of each row
    append?: Struct$id<Partial<STRUCT>>; // add row to bottom, return ids of each row
    update?: Struct$id<Partial<STRUCT>>; // update row by id
    delete?: string; // delete row by id
    sort?: string[]; // sort rows into this order
    undo?: true; // undo last op
    clear?: true; // clear all rows
};

/// A stepper which manages a sequence of data structs, like a table.
/// Critical to note that `struct_validator` has the exact same set of keys as the STRUCT type,
/// which is needed for correct operation in the select* methods
export class BuxStepperStructSequence<STRUCT extends Object> extends BuxStepper<BuxStepperStructSequenceLog<STRUCT>> {
    struct_validator: StructValidator<STRUCT>;

    constructor(schema: string, segment: string, struct_validator: StructValidator<STRUCT>) {
        super(schema, segment);
        this.struct_validator = struct_validator;
    }

    /// Calls select_result and aggerates the errors, or reutrns the clean result
    public select(): Result<Struct$id<STRUCT>[], BuxStepperStructError<STRUCT>[]> {
        let rows = this.select_result();
        let rval: Struct$id<STRUCT>[] = [];
        let errs: BuxStepperStructError<STRUCT>[] = [];
        let sequence = 0;
        for (let row of rows) {
            sequence += 1;
            let newrow = {} as STRUCT;

            for (let key in this.struct_validator) {
                if (row[key].is_ok) {
                    // Due to inclusion of $id, we have to force the type so the parameter works
                    newrow[key] = row[key].value as STRUCT[Extract<keyof STRUCT, string>];
                } else {
                    errs.push({
                        $id: row.$id,
                        sequence,
                        key: key as keyof STRUCT,
                        error: row[key].error,
                    });
                }
            }
            rval.push({ $id: row.$id, ...newrow });
        }

        if (errs.length > 0) {
            return Err(errs);
        } else {
            return Ok(rval);
        }
    }

    public select_result(): Struct$id<StructResult<STRUCT>>[] {
        let rows = this.select_partial();
        return rows.map((row) => {
            let result = {} as StructResult<STRUCT>;
            for (let key in this.struct_validator) {
                // Due to inclusion of $id, we have to force the type so the parameter works
                result[key] = this.struct_validator[key](row[key] as STRUCT[Extract<keyof STRUCT, string>] | undefined);
            }
            return { $id: row.$id, ...result };
        });
    }

    public select_partial(): Struct$id<Partial<STRUCT>>[] {
        let data_map: Map<string, Struct$id<Partial<STRUCT>>> = new Map();
        let data_ord: string[] = [];

        // apply the log to generate the result
        this.logs().forEach((log_row) => {
            // handle all ops first
            if (log_row === "clear") {
                data_map.clear();
                data_ord = [];
                return;
            }

            // then handle the case of an object
            if (log_row.prepend !== undefined) {
                let row = log_row.prepend;
                data_map.set(row.$id, row);
                data_ord.unshift(row.$id);
            }

            if (log_row.append !== undefined) {
                let row = log_row.append;
                data_map.set(row.$id, row);
                data_ord.push(row.$id);
            }

            if (log_row.update !== undefined) {
                const newrow = log_row.update;
                const oldrow = data_map.get(newrow.$id)!;
                data_map.set(newrow.$id, { ...oldrow, ...newrow });
            }

            if (log_row.delete !== undefined) {
                data_map.delete(log_row.delete);
                data_ord = data_ord.filter((id) => id !== log_row.delete);
            }

            if (log_row.sort !== undefined) {
                data_ord = log_row.sort;
            }

            if (log_row.clear !== undefined) {
                data_map.clear();
                data_ord = [];
            }
        });

        return data_ord.map(($id) => {
            return data_map.get($id)!;
        });
    }

    public prepend(row: Partial<STRUCT>): string {
        const $id = uuidv4();
        this.log_push({
            prepend: { $id, ...row },
        });
        return $id;
    }

    public append(row: Partial<STRUCT>): string {
        const $id = uuidv4();
        this.log_push({
            append: { $id, ...row },
        });
        return $id;
    }

    public update($id: string, row: Partial<STRUCT>): void {
        this.log_push({
            update: { $id, ...row },
        });
    }

    public delete($id: string): void {
        this.log_push({
            delete: $id,
        });
    }

    public sort($ids: string[]): void {
        // select the rows, and validate that they are all present
        let rows = this.select_partial();
        let set1 = new Set($ids);
        let set2 = new Set(rows.map((row) => row.$id));

        if (!(set1.size === set2.size && $ids.every((id) => set2.has(id)))) {
            throw new Error("Invalid sort: not all rows are present");
        }

        this.log_push({
            sort: $ids,
        });
    }
}
