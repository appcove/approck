#[approck::http(GET /example/redis1; return HTML;)]
pub mod page {

    pub async fn request(req: Request, doc: Document, redis: Redis) -> Result<Response> {
        // increment a counter in redis
        let counter = redis.incr("counter", 1).await?;

        doc.add_js("./redis1.js");
        doc.add_css("./redis1.css");

        doc.add_body(maud::html!(
            div.container.bg-white {
                a href="/example/" { "◀ Back to Example List" }

                h1 { code {  (req.path()) } }

                hr;

                .redis1 {
                    p.emp1 { "This is a simple example of using Redis." }
                    p { "The counter is currently at: " b {(counter)} }
                }

            }

        ));

        Ok(Response::HTML(doc.into()))
    }
}
