#[approck::http(GET /pricing; return HTML;)]
pub mod page {
    pub async fn request() -> Response {
        Response::HTML("this is our pricing page".to_string().into())
    }
}
