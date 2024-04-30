#[approck::http(GET /about/help; return HTML;)]
pub mod page0 {
    pub async fn request(req: Request, ui: Document) -> Response {
        ui.add_body(maud::html! {

            div.container {
                div.row {
                    div."col-md-3" {
                        div.card {
                            div.card-body.bg-light {
                                h2 {
                                    "Documentation"
                                }
                                ul {
                                    li {
                                        a href="/about/help/1" {
                                            h5 {
                                                "Introduction"
                                            }
                                        }
                                        ul{
                                            li {
                                                a href="/about/help/1" {
                                                    "What is approck?"
                                                }
                                            }
                                            li {
                                                a href="/about/help/1" {
                                                    "Why approck?"
                                                }
                                            }
                                            li {
                                                a href="/about/help/1" {
                                                    "How approck works?"
                                                }
                                            }
                                        }
                                    }
                                }
                                ul {
                                    li {
                                        a href="/about/help/200" {
                                            h5 {
                                                "Quickstart"
                                            }
                                        }
                                        ul{
                                            li {
                                                a href="/about/help/200" {
                                                    "Installation"
                                                }
                                            }
                                            li {
                                                a href="/about/help/200" {
                                                    "Project Structure"
                                                }
                                            }
                                        }
                                    }
                                }
                                ul {
                                    li {
                                        a href="#" {
                                            h5 {
                                                "Examples"
                                            }
                                        }
                                        ul{
                                            li {
                                                a href="#" {
                                                    "Hello World!"
                                                }
                                            }
                                            li {
                                                a href="#" {
                                                    "Form Handling"
                                                }
                                            }
                                            li {
                                                a href="#" {
                                                    "Example API"
                                                }
                                            }
                                        }
                                    }
                                }
                                ul {
                                    li {
                                        a href="#" {
                                            h5 {
                                                "API Reference"
                                            }
                                        }
                                    }
                                    li {
                                        a href="#" {
                                            h5 {
                                                "FAQ"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div."col-md-9" {
                        div.card {
                            div.card-body.bg-light {
                                h3 { "Introduction" }
                                hr;
                                p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }br;
                                h3 { "Audience" }
                                hr;
                                p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." }br;
                                h3 { "Foreword" }
                                hr;
                                p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." }
                                p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." }
                                p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." }
                            }
                        }
                    }
                }
            }
        });

        Response::HTML(ui.into())
    }
}

#[approck::http(GET /about/help/{id:u8}; return HTML;)]
pub mod page {
    pub async fn request(ui: Document) -> Response {
        ui.add_body(maud::html! {
            h1 {
                "Hello!!!"
            }
        });

        Response::HTML(ui.into())
    }
}
