#[approck::http(GET /api/v1/list; return JSON;)]
pub mod data {
    pub async fn request() -> Response {
        Response::JSON("[a,b,c]".into())
    }
}
