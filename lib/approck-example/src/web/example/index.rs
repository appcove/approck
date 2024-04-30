#[approck::http(GET /example/; return HTML;)]
pub mod page {
    pub async fn request(req: Request, ui: Document) -> Response {
        #[rustfmt::skip]
        ui.add_body(maud::html! {
            div.container.bg-white {
                h1 { code {  (req.path()) } }
                hr;
                .row {
                    ."col-md-6" {
                        h2 { "Basic Examples"}
                        ul {
                            li {a href="/example/pathcap1" { "Optional Path Capture 1 (like help topic index)" } }
                            li {a href="/example/post1" { "GET|POST 1 form example with empty post" } }
                            li {a href="/example/post2" { "GET|POST 2 form example with fields" } }
                            li {a href="/example/querystring1" { "Query String 1 with Optional Params" } }
                            li {a href="/example/querystring2" { "Optional Query 2 String with Required Params" } }
                            li {a href="/example/querystring3" { "Query String 3 with Vec and checkboxes" } }
                            li {a href="/example/querystring4" { "Query String 4 with HashSet and checkboxes" } }
                            li {a href="/example/redis1" { "Redis 1" } }
                            li {a href="/example/userlist1" { "User List 1" } }
                            li {a href="/example/websocket1" { "Websocket Example 1" } }
                            li {a href="/example/stream" { "Stream Response Example (4gb download)" } }
                        }
                    }
                    ."col-md-6" {
                        h2 { "Advanced Examples"}
                        ul {
                            li {a href="/example/boxit/" { "boxit" } }
                        }
                    }
                }
            }
        });

        Response::HTML(ui.into())
    }
}
