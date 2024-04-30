#[approck::http(GET /example/querystring2?a=i32&b=u32&c=String; return HTML;)]
pub mod page {
    pub async fn request(req: Request, ui: Document, qs: Option<QueryString>) -> Response {
        #[rustfmt::skip]
        ui.add_body(maud::html! {
            div.container.bg-white {
                a href="/example/" { "◂ Back to Example List" } 
                h1 { code {  (req.path()) } }
    
                @match qs {
                    Some(qs) => {
                        ul {
                            li { "Query String a: " (format!("{:?}", qs.a)) }
                            li { "Query String b: " (format!("{:?}", qs.b)) }
                            li { "Query String c: " (format!("{:?}", qs.c)) }
                        }
                    }
                    None => {
                        p { "No Query String" }
                    }
                }
    
                hr;
                ul{
                    li {a href="/example/querystring2" { "/example/querystring2" } }
                }
                hr;
                ul {
                    li {a href="/example/querystring2?a=1&b=2&c=256" { "/example/querystring2?a=1&b=2&c=256" } }
                }
            }
        });

        Response::HTML(ui.into())
    }
}
