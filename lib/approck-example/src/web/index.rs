#[approck::http(GET /; return HTML;)]
pub mod page {
    pub async fn request(ui: Document) -> Response {
        ui.add_js("./index.js");

        #[rustfmt::skip]
        ui.add_body(maud::html! {

            canvas id="canvas" style="width: 100%; height: 600px;" {}

            // canvas #canvas style="width: 100%; height: 600px;";
            div."home-white" {
                div.container {
                    div.home-section {
                        div {
                            title { "Lorem Ipsum Framework" }
                            // Add your CSS and other head elements here if needed
                        }
                        div {
                            div."text-center py-3" {
                                div {
                                    h1 { "Lorem Ipsum" }
                                    p {
                                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit" br;
                                        "sed do eiusmod tempor incididunt ut labore et dolore magna aliqua" br;
                                        "enim ad minim veniam, quis nostrud exercitation ullamco laboris."
                                    }
                                }
                                div {
                                    a href="#" {
                                        button.btn.btn-primary."m-1" { "Get Started" }
                                    }
                                    a href="#" {
                                        button.btn.btn-outline-secondary."m-1" { "Learn More" }
                                    }
                                    a href="#" {
                                        button.btn.btn-outline-secondary."m-1" { "Read FAQ" }
                                    }
                                }
                            }
                            hr;
                            div."my-4" {
                                div."row text-center" {
                                    div."col-md-3" {
                                        div."card h-100" {
                                            div."card-body".bg-teal {
                                                img src="https://d1lkraw6keepp8.cloudfront.net/8583f262ab6e/assets/art/clouds-c8ac93045ba9068c7def69366249216c08f7b8ab554b04b2b0b9db7fb0888b98.svg" loading="" alt="";
                                                h5."pt-2" { "Lorem ipsum" }
                                                p class="card-text" { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }
                                                a href="#" class="btn btn-primary" { "Get Started" }
                                            }
                                        }
                                    }
                                    div."col-md-3" {
                                        div."card h-100" {
                                            div."card-body".bg-teal {
                                                img src="https://d1lkraw6keepp8.cloudfront.net/8583f262ab6e/assets/art/diamond-3ed5cbea0f9ae75c1e22fecae23093347948d646c5a073dfd401d6dda140ba68.svg" loading="" alt="";
                                                h5."pt-2" { "Lorem ipsum" }
                                                p."card-text" { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }
                                                a href="#" class="btn btn-secondary" { "Learn More" }
                                            }
                                        }
                                    }
                                    div."col-md-3" {
                                        div."card h-100" {
                                            div."card-body".bg-teal {
                                                img src="https://d1lkraw6keepp8.cloudfront.net/8583f262ab6e/assets/art/thumbsup-4ce98bad96d71f307eee97afffc7e035f308ffc1a686f755f77d6e3e961fcc63.svg" loading="" alt="";
                                                h5."pt-2" { "Lorem ipsum" }
                                                p."card-text" { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }
                                                a href="#" class="btn btn-secondary" { "See Examples" }
                                            }
                                        }
                                    }
                                    div."col-md-3" {
                                        div."card h-100" {
                                            div."card-body".bg-teal {
                                                img src="https://d1lkraw6keepp8.cloudfront.net/8583f262ab6e/assets/art/castle-88bf958b6eec4a095da6ba9fd57d3f2b1dabe77a052df81955ab62e1ec4d71bd.svg" loading="" alt="";
                                                h5."pt-2" { "Lorem ipsum" }
                                                p."card-text" { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }
                                                a href="#" class="btn btn-secondary" { "See How" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div."home-gray" {
                div.container {
                    div.home-section."my-4" {
                        div.row {
                            div."col-md-6" {
                                div."card.h-100.p-3" {
                                    img src="https://code.visualstudio.com/assets/docs/languages/rust/hover.png" alt="" class="img-vscode";
                                }
                            }
                            div."col-md-6" {
                                div."card.h-100.p-3" {
                                    h2 { "Hello, Lorem ipsum!" }
                                    hr;
                                    p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit." }
                                    p { "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." }
                                    p { "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum." }
                                    p { "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum." }
                                }
                            }
                        }
                    }
            
                }
            }
            div."home-white" {
                div.container {
                    div.home-section {
                        div.row {
                            div."col-md-6" {
                                div."card.h-100.p-3" {
                                    h2 { "Hello, Lorem ipsum!" }
                                    hr;
                                    p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit." }
                                    p { "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." }
                                    p { "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum." }
                                    p { "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum." }
                                }
                            }
                            div."col-md-6" {
                                div."card.h-100.p-3" {
                                    img src="https://global.discourse-cdn.com/business5/uploads/rust_lang/original/3X/8/e/8e38683e854a4d35d73e309833f797caa137ed9a.png" alt="" class="img-vscode";
                                }
                            }
                        }
                    }
                }
            }
            div."home-gray" {
                div.container {
                    div.home-section {
                        div.row {
                            div."col-md-6" {
                                div."card.h-100.p-3" {
                                    img src="https://about.gitlab.com/images/blogimages/learn-rust-with-ai-code-suggestions-getting-started/learn_rust_ai_gitlab_code_suggestions_print_variable_first.png" alt="" class="img-vscode";
                                }
                            }
                            div."col-md-6" {
                                div."card.h-100.p-3" {
                                    h2 { "Hello, Lorem ipsum!" }
                                    hr;
                                    p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit." }
                                    p { "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." }
                                    p { "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum." }
                                    p { "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum." }
                                }
                            }
                        }
                    }
                }
            }
            div."home-white" {
                div.container {
                    div.home-section {
                        div.row {
                            div."col-md-6" {
                                div."card.h-100.p-3" {
                                    h2 { "Hello, Lorem ipsum!" }
                                    hr;
                                    p { "Lorem ipsum dolor sit amet, consectetur adipiscing elit." }
                                    p { "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." }
                                    p { "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum." }
                                    p { "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum." }
                                }
                            }
                            div."col-md-6" {
                                div."card.h-100.p-3" {
                                    img src="https://global.discourse-cdn.com/business5/uploads/rust_lang/original/3X/8/e/8e38683e854a4d35d73e309833f797caa137ed9a.png" alt="" class="img-vscode";
                                }
                            }
                        }
                    }
                }
            }
            div."home-gray" {
                div.container {
                    div.home-section {
                        div.d-flex.align-items-center.justify-content-center.flcolumn {
                            div."col-md-9" {
                                div.panel.panel-default {
                                    div.panel-body {
                                        h1."text-center mb-4" {"About Us"}
                                        p."mb-3" {"
                                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed tempor justo sit amet ligula auctor, eget scelerisque eros tincidunt. Nulla facilisi. Sed ultricies, velit sed laoreet tincidunt, nulla augue dictum tellus, at tincidunt nulla quam non ex. Vivamus nec sapien vel ante cursus aliquet. Vivamus vel urna sed odio malesuada dignissim. Duis luctus quam nec congue condimentum. Integer id risus nec arcu hendrerit pharetra."}
            
                                        p."mb-0" {"
                                        Sed pharetra ex in augue convallis, a posuere massa bibendum. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Maecenas bibendum dui sit amet aliquet fringilla. Vivamus euismod vestibulum urna, id egestas erat malesuada eu. Sed eget arcu non risus malesuada tincidunt. Nullam ullamcorper, velit nec efficitur feugiat, elit purus egestas risus, at cursus sapien libero eget quam. Vivamus at purus eu quam congue congue eu non lacus. Vivamus id justo et turpis blandit sodales."}
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div."home-white" {
                div.container {
                    div.home-section {
                        div.row."pt-5" {
                            div."col-lg-4".text-center {
                                img height="60" src="https://d1lkraw6keepp8.cloudfront.net/8583f262ab6e/assets/art/handshake-a9a9c8833ed0d7b3d279377ac647d6a4aba179bbbb7937552467598615d20f98.svg" {}
                                h5."my-3" { "LOREM IPSUM" }
                                p.lead { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed tempor justo sit amet ligula auctor, eget scelerisque eros tincidunt" }
                            }
                            div."col-lg-4".text-center {
                                img height="60" src="https://d1lkraw6keepp8.cloudfront.net/8583f262ab6e/assets/art/connections-afd7b465f57f5ff83ca7f6ca45d73c33af3ef7727d1655ecf5f7acfbd6f39977.svg" {}
                                h5."my-3" { "LOREM IPSUM" }
                                p.lead { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed tempor justo sit amet ligula auctor, eget scelerisque eros tincidunt" }
                            }
                            div."col-lg-4".text-center {
                                img height="60" src="https://d1lkraw6keepp8.cloudfront.net/8583f262ab6e/assets/art/castle-88bf958b6eec4a095da6ba9fd57d3f2b1dabe77a052df81955ab62e1ec4d71bd.svg" {}
                                h5."my-3" { "LOREM IPSUM" }
                                p.lead { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed tempor justo sit amet ligula auctor, eget scelerisque eros tincidunt" }
                            }
                        }
                    }
                }
            }
            div."home-gray" {
                div.container {
                    div.home-section {
                        p.lead."text-center" { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed tempor justo sit amet ligula auctor, eget scelerisque eros tincidunt.Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed tempor justo sit amet ligula auctor, eget scelerisque eros tincidunt.Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed tempor justo sit amet ligula auctor, eget scelerisque eros tincidunt" }
                        "---------------------------------------------------"
                        div."heart-icon" { "❤️" }
                        div."thank-you-text" { "Thank You!" }
                    }
                }
            }
        });
        Response::HTML(ui.into())
    }
}
