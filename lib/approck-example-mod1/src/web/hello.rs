#[approck::http(GET /hello-world; return HTML;)]
pub mod page {
    use granite_postgres::DB;

    pub async fn request(db: Postgres, doc: Document) -> Response {
        let v = db.query_one("SELECT NOW()::text", &[]).await.unwrap();
        let time: String = v.get(0);
        doc.add_js("/hello.js");
        doc.add_body(maud::html! {
            h1 { "Hello, world!" }
            p { (time) }
        });

        Response::HTML(doc.into())
    }
}
