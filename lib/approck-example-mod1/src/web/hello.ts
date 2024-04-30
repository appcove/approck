import { set_moon_brightness } from "@approck-example-mod2/moon_module.mjs";

export function hello() {
    console.log("Hello, world!");
    console.log("Setting moon brightness to 0.5");
    set_moon_brightness(0.5);
}
