import { BuxStepper } from "./mod.mjs";

type BuxStepperValueLog<VALUE> = {
    update?: VALUE;
};

export class BuxStepperValue<VALUE> extends BuxStepper<BuxStepperValueLog<VALUE>> {
    update(value: VALUE) {
        this.log_push({ update: value });
    }

    select(): VALUE | undefined {
        let value = undefined;
        this.logs().forEach((data) => {
            // handle all ops first
            if (data === "clear") {
                value = undefined;
                return; // CONTINUE
            }

            // otherwise handle an object

            if (data.update !== undefined) {
                value = data.update;
            }
        });
        return value;
    }
}
