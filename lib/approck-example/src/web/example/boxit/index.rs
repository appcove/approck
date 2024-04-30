#[approck::http(GET /example/boxit/; return HTML;)]
pub mod page {
    pub async fn request(req: Request, ui: Document) -> Response {
        ui.add_js("./index.js");

        #[rustfmt::skip]
        ui.add_body(maud::html! {
            div.container.bg-white {
                a href="/example/" { "◂ Back to Example List" } 
                h1 { code {  (req.path()) } }
                hr;
                canvas id="canvas" width="2000" height="800" style="border:1px solid #000000; width: 100%; height: auto;";
            }
        });

        Response::HTML(ui.into())
    }
}
