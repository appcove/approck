import type { BuxDocument } from "@crate/document/mod.mjs";
import { DASH } from "@granite/util.mjs";

type LogRow<LOG_DATA> = [
    number, // timestamp
    LOG_DATA | "undo" | "clear",
];

export abstract class BuxStepperGroup {
}

export abstract class BuxStepper<LOG_DATA> {
    private log_data: LogRow<LOG_DATA>[] = [];
    private schema: string;
    private segment: string;

    private session_storage_log_key: string;

    constructor(schema: string, segment: string) {
        this.schema = schema;
        this.segment = segment;
        this.session_storage_log_key = `${DASH(this.schema)}:${DASH(this.segment)}:log`;

        // Attempt to load the log from session storage
        try {
            const stored = sessionStorage.getItem(this.session_storage_log_key);
            if (stored !== null) {
                this.log_data = JSON.parse(stored);
            }
        } catch (e) {
            console.error("Error loading session storage key", this.session_storage_log_key, e);
        }
    }

    protected log_push(data: LOG_DATA | "undo" | "clear"): void {
        this.log_data.push([
            Date.now(),
            data,
        ]);
        this.save_to_session_storage();
    }

    private log_pop(): void {
        this.log_data.pop();
        this.save_to_session_storage();
    }

    private log_reset(): void {
        this.log_data = [];
        this.save_to_session_storage();
    }

    /// call this to save the contents of the specified segment to session storage
    private save_to_session_storage() {
        sessionStorage.setItem(this.session_storage_log_key, JSON.stringify(this.log_data));
    }

    logs(): (LOG_DATA | "clear")[] {
        const rval: (LOG_DATA | "clear")[] = [];
        this.log_data.forEach((log) => {
            if (log[1] === "undo") {
                rval.pop();
            } else {
                rval.push(log[1]);
            }
        });

        return rval;
    }

    /// Check to see if there is anything to undo, and if so, push an undo operation
    public undo(): void {
        if (this.logs().length > 0) {
            this.log_push("undo");
        }
    }

    /// Check if the last item on the raw logs was an undo, and if so, pop it off
    public redo(): void {
        // examine the last element to see if it was an undo
        if (this.log_data.length > 0 && this.log_data[this.log_data.length - 1][1] === "undo") {
            this.log_pop();
        }
    }

    /// Check if there is anything to clear, and if so, push a clear operation
    public clear(): void {
        if (this.logs().length > 0) {
            this.log_push("clear");
        }
    }
}
