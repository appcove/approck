export function resize_canvas_and_get_context(id: string): CanvasRenderingContext2D {
    const canvas = document.getElementById(id) as HTMLCanvasElement;

    // raise exception if canvas is null
    if (!canvas) {
        throw new Error("Could not get canvas");
    }

    // check the element type
    if (!(canvas instanceof HTMLCanvasElement)) {
        throw new Error("Element is not a canvas");
    }

    // get the context
    const context = canvas.getContext("2d");

    if (!context) {
        throw new Error("Could not get 2d context");
    }

    // Ratio for high DPI displays
    var ratio = window.devicePixelRatio || 1;

    // Set the canvas size to the actual pixel size
    canvas.width = canvas.clientWidth * ratio;
    canvas.height = canvas.clientHeight * ratio;

    // Scale all drawing to the high DPI size
    context.scale(ratio, ratio);

    return context;
}
