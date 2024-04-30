#[approck::http(GET /calculator/; return HTML;)]
pub mod page {
    pub async fn request(doc: Document) -> Response {
        doc.add_js("/calculator/index.js");
        Response::HTML(doc.into())
    }
}
