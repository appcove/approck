#[approck::http(GET /asset/logo.svg; return SVG;)]
pub mod logo_svg {
    pub async fn request() -> Response {
        Response::SVG(include_str!("logo.svg").into())
    }
}
