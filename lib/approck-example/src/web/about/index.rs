#[approck::http(GET /about/; return Text;)]
pub mod page {
    pub async fn request(_req: Request) -> Response {
        Response::Text("this is about our website".into())
    }
}
