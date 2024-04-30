type action_log_record = {
    id: number;
    timestamp: number;
};

export class Example {
    private idb: IDBDatabase;

    private constructor(idb: IDBDatabase) {
        this.idb = idb;
    }

    /// open (create if needed) a database with the name given by the `database` parameter
    /// example database names include:
    ///  * `foo_step1`
    ///  * `foo_step2`
    ///  * `account_edit`
    static async open(database: string): Promise<Example> {
        database = `bux-stepper-${database}`;

        let promise: Promise<IDBDatabase> = new Promise((resolve, reject) => {
            let request = indexedDB.open(database, 1);

            request.onupgradeneeded = (event) => {
                let db = request.result;
                db.createObjectStore("current_state", { keyPath: "id", autoIncrement: true });
                db.createObjectStore("event_log", { keyPath: "id", autoIncrement: true });
            };

            request.onsuccess = (event) => {
                console.log("Database opened successfully");
                resolve(request.result);
            };

            request.onerror = (event) => {
                reject(event);
            };
        });

        let idb: IDBDatabase = await promise;

        return new Example(idb);
    }

    async write(): Promise<number> {
        // cannot include the `id` field if you want it to auto-increment
        let record: Omit<action_log_record, "id"> = {
            timestamp: Date.now(),
        };

        return new Promise((resolve, reject) => {
            const transaction = this.idb.transaction(["action_log"], "readwrite");
            const store = transaction.objectStore("action_log");
            const request = store.add(record);

            request.onsuccess = () => resolve(request.result as number);
            request.onerror = () => reject(request.error);
        });
    }
}
