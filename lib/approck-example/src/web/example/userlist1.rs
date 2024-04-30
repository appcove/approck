#[approck::http(GET /example/userlist1; return HTML;)]
pub mod page {
    pub async fn request(req: Request, doc: Document, db: Postgres) -> Result<Response> {
        // Call it with a connection
        let mut user_list = crate::user::list(&db).await?;
        let user_count;

        {
            let mut dbtx = db.transaction().await?;

            // nested transaction support
            {
                let dbtx2 = dbtx.transaction().await?;
                dbtx2.rollback().await.unwrap();
            }

            user_count = granite::pg_row!(
                db = dbtx;
                row = {
                    count: i64,
                };
                SELECT
                    count(*)
                FROM
                    user
            )
            .await?;

            // call it with a transaction
            user_list.extend(crate::user::list(&dbtx).await?);

            // rollback the transaction
            dbtx.rollback().await?;
        }

        // call it with a connection again
        user_list.extend(crate::user::list(&db).await?);

        #[rustfmt::skip]
        doc.add_body(maud::html! {
            div.container.bg-white {
                a href="/example/" { "◀ Back to Example List" } 

                h1 { code {  (req.path()) } }
    
                hr;

                h2 { "User Count: " (user_count.count) }

                hr; 

                h2 { "User List" }
                ul {
                    @for user in user_list.iter() {
                        li { (user.first_name) }
                    }
                }
            }
        });

        Ok(Response::HTML(doc.into()))
    }
}
