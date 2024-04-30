#[approck::http(GET /example/websocket1; return HTML|WebSocketUpgrade;)]
pub mod page {
    pub async fn request(req: Request, ui: Document) -> Response {
        // An example of moving a variable into a closure which is used to produce the
        // websocket_handler future.
        let name = "jason";
        match req
            .upgrade_to_websocket(move |socket| async move {
                println!("{name}");
                self::websocket(socket).await
            })
            .await
        {
            Ok(Some(response)) => return Response::WebSocketUpgrade(response),
            Err(_error) => todo!(),
            _ => {}
        }

        ui.add_js("./websocket1.js");

        ui.add_body(maud::html! {
            div.container.bg-white {
                a href="/example/" { "◂ Back to Example List" }
                h1 { "WebSocket Example" }
                hr;
                p { "This example demonstrates a simple WebSocket connection." }
                p { "Open the browser console to see the messages." }
                hr;
                button #send-foo-bar { "Send FooBar" }
                hr;
                div #output {}
            }
        });

        Response::HTML(ui.into())
    }

    async fn websocket(mut websocket: WebSocket) {
        websocket
            .send("Hello from the server!".into())
            .await
            .unwrap();

        while let Some(message) = websocket.recv().await {
            let message = match message {
                Ok(message) => message,
                Err(error) => {
                    eprintln!("Error: {error:?}");
                    continue;
                }
            };
            match message.into_data() {
                WebSocketMessageData::Text(text) => {
                    println!("Received: {}", text);
                    websocket
                        .send(format!("Got this on server: {:?}", text).into())
                        .await
                        .unwrap();
                }
                WebSocketMessageData::Close => {
                    println!("Connection closed");
                }
                _ => {}
            }
        }
    }
}
