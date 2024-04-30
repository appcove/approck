#[approck::http(GET|POST /example/post1; return HTML;)]
pub mod page {
    pub async fn request(req: Request, ui: Document) -> Response {
        #[rustfmt::skip]
        ui.add_body(maud::html! {
            div.container.bg-white {
                a href="/example/" { "◂ Back to Example List" } 
                h1 { code {  (req.path()) } }

                hr;

                @if req.is_post() {
                    h2 { "POST" }
                }
                @if req.is_get() {
                    h2 { "GET" }
                }

                hr;
                form method="post" action=(req.path()) class="g-3" {
                    div class="mb-3" {
                        a.btn.btn-primary href=(req.path()) { "Get" }
                        " "
                        button.btn.btn-primary type="submit"  { "Post" } 
                    }
                }
            }
        });

        Response::HTML(ui.into())
    }
}
