#[approck::http(GET|POST /example/post2; return HTML|Redirect;)]
pub mod page {
    #[allow(dead_code)]
    pub struct PostForm {
        name: String,
        email: String,
        message: String,
    }

    pub async fn request(req: Request, ui: Document, form: Option<PostForm>) -> Response {
        let form = match form {
            Some(form) => form,
            None => PostForm {
                name: "".to_string(),
                email: "".to_string(),
                message: "".to_string(),
            },
        };

        if form.message == "redirect" {
            return Response::Redirect("/example/".into());
        }

        #[rustfmt::skip]
        ui.add_body(maud::html! {
            div.container.bg-white {
                a href="/example/" { "◂ Back to Example List" } 
                h1 { code {  (req.path()) } }

                hr;

                @if req.is_post() {
                    h2 { "post" }
                    ul {
                        li { "Name: " (form.name) }
                        li { "Email: " (form.email) }
                        li { "Message: " (form.message) }
                    }
                }
                @else {
                    h2 { "not post" }
                }

                hr;
                p {
                    "In the following form, if you type the word `redirect` into the message field and submit the form, you will be redirected to a different page."
                }
                form method="post" action=(req.path()) class="g-3" {
                    div class="mb-3" {
                        labe.form-label for="name" { "Name" }
                        input.form-control type="text" name="name" placeholder="Name" required="true" value=(form.name) {};
                    }
                    div class="mb-3" {
                        label for="email" class="form-label" { "Email" }
                        input type="email" class="form-control" id="email" name="email" placeholder="Email" required="true" value=(form.email) {}
                    }
                    // textarea
                    div class="mb-3" {
                        label for="message" class="form-label" { "Message" }
                        textarea class="form-control" id="message" name="message" rows="3" placeholder="Message" required="true" {(form.message)}
                    }
                    div class="mb-3" {
                        input type="submit" class="btn btn-primary" value="Submit";
                    }
                }
            }
        });

        Response::HTML(ui.into())
    }
}
