pub mod help;
pub mod index;

#[approck::http(GET /about; return Redirect;)]
pub mod redirect {
    pub async fn request() -> Response {
        Response::Redirect("/about/".into())
    }
}
