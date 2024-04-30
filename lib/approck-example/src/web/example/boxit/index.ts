export class Canvas {
    $canvas: HTMLCanvasElement;
    $ctx: CanvasRenderingContext2D;
    width: number;
    height: number;

    constructor(eid: string) {
        this.$canvas = document.getElementById(eid) as HTMLCanvasElement;
        const $ctx = this.$canvas.getContext("2d");
        if ($ctx === null) {
            throw new Error("Could not get canvas context");
        }
        this.$ctx = $ctx;
        this.width = this.$canvas.width;
        this.height = this.$canvas.height;
    }

    draw(boxes: Box[]): void {
        this.$ctx.resetTransform();
        this.$ctx.clearRect(0, 0, this.width, this.height);

        // draw the boxes
        for (const box of boxes) {
            this.$ctx.fillStyle = box.color;
            this.$ctx.fillRect(box.x, box.y, box.width, box.height);
        }

        // draw a message in the middle
        this.$ctx.font = "30px Arial";
        this.$ctx.textAlign = "center";
        this.$ctx.textBaseline = "middle";
        this.$ctx.fillStyle = "#444444";
        const text = `Boxit Demo with ${boxes.length} boxes!`;
        this.$ctx.fillText(text, this.width / 2, 30);
    }
}

type Box = {
    x: number;
    y: number;
    width: number;
    height: number;
    color: string;
};

const boxes: Box[] = [];

const canvas = new Canvas("canvas");
let delay = 10;

function ticker() {
    // Perform AJAX call to ./ajax/random-xy to get the box data
    fetch("/example/boxit/ajax/random-xy")
        .then(response => response.json())
        .then(data => {
            // Assuming the data is in the correct format and directly usable
            boxes.push(data);

            // if > 1000 boxes, remove the first one
            if (boxes.length > 3600) {
                boxes.shift();
            }

            // Draw the boxes
            canvas.draw(boxes);
        })
        .catch(error => console.error("Error fetching box data:", error));

    // increment delay
    delay += 1;

    // Schedule the next tick
    setTimeout(ticker, delay);
}

ticker();
