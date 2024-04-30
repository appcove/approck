#[approck::http(GET /example/name?first=String; return HTML;)]
pub mod page {
    pub async fn request(ui: Document) -> Response {
        #[rustfmt::skip]
        ui.add_body(maud::html! {
            div.container.bg-white {
                form method="GET" action="/example/name" {
                    div {
                        label { "First Name"}
                        input.form-control type="text" name="first_name";
                    }
                    div {
                        label { "Last Name"}
                        input.form-control type="text" name="last_name";
                    }
                    
                    div {
                        button type="submit" { "Submit" }
                        a .btn.btn-primary href="?" { "Reset" }
                    }
                }
            }
        });

        Response::HTML(ui.into())
    }
}
