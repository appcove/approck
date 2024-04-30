// create a websocket to /example/websocket1/ws
const ws = new WebSocket(window.location.href.replace(/^https:/, "wss:"));
const $output = document.getElementById("output") as HTMLDivElement;
const $send_foo_bar = document.getElementById("send-foo-bar") as HTMLButtonElement;

// print any messages
ws.onmessage = function(event) {
    console.log(event.data);
    let p = document.createElement("p");
    p.textContent = event.data;
    $output.appendChild(p);
};

ws.onerror = function(event) {
    console.error(event);
};

// send a message when the button is clicked
$send_foo_bar.onclick = function() {
    ws.send("foo bar");
};
