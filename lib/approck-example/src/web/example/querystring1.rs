#[approck::http(GET /example/querystring1?a=Option<i32>&b=Option<u32>&c=Option<String>; return HTML;)]
pub mod page {
    pub async fn request(req: Request, ui: Document, qs: QueryString) -> Response {
        #[rustfmt::skip]
        ui.add_body(maud::html! {
            div.container.bg-white {
                a href="/example/" { "◀ Back to Example List" } 

                h1 { code {  (req.path()) } }
    
                ul {
                    li { "Query String a: " (format!("{:?}", qs.a)) }
                    li { "Query String b: " (format!("{:?}", qs.b)) }
                    li { "Query String c: " (format!("{:?}", qs.c)) }
                }
    
                hr;
                ul{
                    li {a href="/example/querystring1" { "/example/querystring1" } }
                }
                hr;
                ul {
                    li {a href="/example/querystring1?a=1" { "/example/querystring1?a=1" } }
                    li {a href="/example/querystring1?a=1&b=2" { "/example/querystring1?a=1&b=2" } }
                    li {a href="/example/querystring1?a=1&b=2&c=3" { "/example/querystring1?a=1&b=2&c=3" } }
                    li {a href="/example/querystring1?a=1&b=2&c=256" { "/example/querystring1?a=1&b=2&c=256" } }
                }
            }
        });

        Response::HTML(ui.into())
    }
}
