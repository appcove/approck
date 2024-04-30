#[approck::http(GET /example/pathcap1/{pc:String}; return HTML;)]
pub mod page_with_capture {
    pub async fn request(req: Request, ui: Document, path: Path) -> Response {
        match super::page::request(req, ui, Some(path.pc)).await {
            super::page::Response::HTML(html) => Response::HTML(html),
        }
    }
}

#[approck::http(GET /example/pathcap1; return HTML;)]
pub mod page {
    pub async fn request(req: Request, ui: Document, pc: Option<String>) -> Response {
        #[rustfmt::skip]
        ui.add_body(maud::html! {
            div.container.bg-white {
                a href="/example/" { "◂ Back to Example List" } 
                h1 { code { (req.path()) } }
    
                @match &pc {
                    Some(pc) => {
                        p { "Captured Path: " (pc) }
                    }
                    None => {
                        p { "Captured Path: None" }
                    }
                }
    
                hr;
                ul{
                    li {a href="/example/pathcap1" { "/example/pathcap1" } }
                }
                hr;
                ul {
                    li {a href="/example/pathcap1/a" { "/example/pathcap1/a" } }
                    li {a href="/example/pathcap1/b" { "/example/pathcap1/b" } }
                    li {a href="/example/pathcap1/c" { "/example/pathcap1/c" } }
                }
              
    
            }
        });

        Response::HTML(ui.into())
    }
}
