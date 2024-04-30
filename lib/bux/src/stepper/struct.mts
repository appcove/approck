import { Err, Ok, Result, Struct$id, StructResult, StructValidator } from "@granite/mod.mjs";
import { BuxStepper } from "./mod.mjs";

type BuxStepperStructError<STRUCT> = {
    key: keyof STRUCT;
    error: string;
};

type BuxStepperStructLog<STRUCT> = {
    update?: Partial<STRUCT>; // update row
};

/// A stepper which manages a sequence of data structs, like a table.
/// Critical to note that `struct_validator` has the exact same set of keys as the STRUCT type,
/// which is needed for correct operation in the select* methods
export class BuxStepperStruct<STRUCT extends Object> extends BuxStepper<BuxStepperStructLog<STRUCT>> {
    struct_validator: StructValidator<STRUCT>;

    constructor(schema: string, segment: string, struct_validator: StructValidator<STRUCT>) {
        super(schema, segment);
        this.struct_validator = struct_validator;
    }

    /// Calls select_result and aggerates the errors, or reutrns the clean result
    /*
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
    /*

    /*
    public select_result(): StructResult<STRUCT>[] {
        let result = {} as StructResult<STRUCT>;
        for (let key in this.struct_validator) {
            // Due to inclusion of $id, we have to force the type so the parameter works
            result[key] = this.struct_validator[key](row[key] as STRUCT[Extract<keyof STRUCT, string>] | undefined);
        }
        return result;
    }
    */

    public get_partial(): Partial<STRUCT> {
        let rval: Partial<STRUCT> = {};

        // apply the log to generate the result
        this.logs().forEach((log_row) => {
            // handle all ops first
            if (log_row === "clear") {
                rval = {};
                return;
            }

            // then handle the case of an object
            if (log_row.update !== undefined) {
                rval = { ...rval, ...log_row.update };
            }
        });

        return rval;
    }

    public update(data: Partial<STRUCT>): void {
        this.log_push({
            update: data,
        });
    }
}
