import "@crate/toolbar/mod.mjs";

import { BuxStepperStructSequence } from "@crate/stepper/struct_sequence.mjs";
import { BuxToolbar } from "@crate/toolbar/mod.mjs";
import { BuxElementGroup } from "./group.mjs";
import { BuxElement } from "./mod.mjs";

export abstract class BuxElementGroupSequence<
    STRUCT_INPUT_ELEMENT extends BuxElementGroup<STRUCT>,
    STRUCT extends object,
> extends BuxElement {
    private stepper!: BuxStepperStructSequence<STRUCT>;
    private toolbar!: BuxToolbar;
    private $groups!: HTMLDivElement;

    abstract row_element_name(): string;

    /// must be called before using the object
    public init(stepper: BuxStepperStructSequence<STRUCT>) {
        this.stepper = stepper;
        this.toolbar = document.createElement("bux-toolbar") as BuxToolbar;
        this.$groups = document.createElement("div");

        this.toolbar.add_undo(() => {
            this.stepper.undo();
            this.reload();
        });

        this.toolbar.add_redo(() => {
            this.stepper.redo();
            this.reload();
        });

        this.toolbar.add_clear(() => {
            this.stepper.clear();
            this.reload();
        });

        this.appendChild(this.toolbar);
        this.appendChild(this.$groups);
    }

    public reload() {
        let data = this.stepper.select_partial();
        this.$groups.innerHTML = "";
        data.forEach((row) => {
            let $row = this.append_group(row.$id, row) as STRUCT_INPUT_ELEMENT;
            $row.on_remove = () => {
                this.stepper!.delete(row.$id);
                this.reload();
            };
        });
    }

    public append_new(): string {
        const row = {};
        const $id = this.stepper.append({});
        this.append_group($id, row);
        return $id;
    }

    public append_group($id: string, data: Partial<STRUCT>): STRUCT_INPUT_ELEMENT {
        const $group = document.createElement(this.row_element_name()) as STRUCT_INPUT_ELEMENT;
        $group.setAttribute("id", $id);

        $group.on_change = (row) => {
            this.stepper.update($id, row);
        };

        $group.on_remove = () => {
            this.stepper.delete($id);
            $group.remove();
        };

        if (data !== null) {
            $group.value = data;
        }

        // TODO: right about here, look in a particular place to see if this row id has an error struct associated with it, and
        // if so, pass that error struct into the newly constructed row after setting it's value

        this.$groups.appendChild($group);

        return $group;
    }

    public validate() {
        Array.from(this.$groups.childNodes).forEach(($row) => {
            ($row as STRUCT_INPUT_ELEMENT).validate();
        });
    }

    public get value(): Partial<STRUCT>[] {
        return Array.from(this.$groups.childNodes).map(($group) => ($group as STRUCT_INPUT_ELEMENT).value);
    }

    public get length(): number {
        return this.$groups.childNodes.length;
    }
}
