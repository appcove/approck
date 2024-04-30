import { resize_canvas_and_get_context } from "@crate/canvas_util.mjs";

window.addEventListener("resize", () => {
    resize_canvas_and_get_context("canvas");
});

const ctx = resize_canvas_and_get_context("canvas");

ctx.fillStyle = "red";
ctx.fillRect(10, 10, 100, 100);

ctx.fillStyle = "blue";
ctx.fillRect(150, 150, 100, 100);
