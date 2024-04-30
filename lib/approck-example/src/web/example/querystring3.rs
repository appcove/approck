#[approck::http(GET /example/querystring3?a=Vec<u32>; return HTML;)]
pub mod page {
    pub async fn request(req: Request, ui: Document, qs: QueryString) -> Response {
        #[rustfmt::skip]
        ui.add_body(maud::html! {
            div.container.bg-white {
                a href="/example/" { "◂ Back to Example List" } 
                h1 { code {  (req.path()) } }
    
                
                code { "Query String a: " (format!("{:?}", qs.a)) }
                        
                hr;
                ul{
                    li {a href="/example/querystring3" { "/example/querystring3" } }
                    li {a href="/example/querystring3?a=1&a=10&a=100" { "/example/querystring3?a=1&a=10&a=100" } }
                    li {a href="/example/querystring3?a=1&a=10&a=100&a=1000" { "/example/querystring3?a=1&a=10&a=100&a=1000" } }
                }

                hr;
                h2 {"form example"}
                form method="get" action=(req.path()) {
                    
                    // 1-4
                    @for i in 1..=12 {
                        input type="checkbox" name="a" value=(i) checked[qs.a.contains(&i)]; (maud::PreEscaped("&nbsp")); (format!("value is {}", i)); br;
                    }

                    input type="submit" value="Submit";
                }
            }
        });

        Response::HTML(ui.into())
    }
}
