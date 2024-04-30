#[approck::http(GET /approck-example-mod2; return HTML;)]
pub mod page {
    pub async fn request(_req: Request) -> Response {
        Response::HTML(HTML::new(
            r#"
                <html>
                    <head>
                        <title>Hello Moon</title>
                    </head>
                    <body>
                        <h1>Hello Moon</h1>
                    </body>
                </html>
            "#
            .to_string(),
        ))
    }
}
