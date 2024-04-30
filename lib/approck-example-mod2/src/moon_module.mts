import { Object } from "ts-toolbelt";

type Moon = {
    brightness: number;
    color: string;
    phase: string;
    position: string;
};

type NullableMoon = Object.Nullable<Moon>;

export function set_moon_brightness(brightness: number) {
    console.log(`Moon brightness set to ${brightness}`);
    let moon: NullableMoon = {
        brightness: brightness,
        color: null,
        phase: null,
        position: null,
    };

    console.log(moon);
}

export function set_moon_color(color: string) {
    console.log(`Moon color set to ${color}`);
}

export function set_moon_phase(phase: string) {
    console.log(`Moon phase set to ${phase}`);
}

export function set_moon_position(position: string) {
    console.log(`Moon position set to ${position}`);
}
