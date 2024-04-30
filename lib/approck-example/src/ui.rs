use maud::{html, Markup, Render};

// write a struct to represent an html template
pub struct Document {
    title: Option<String>,
    head: Vec<Markup>,
    body: Vec<Markup>,
    tail: Vec<Markup>,
    js_list: Vec<String>,
    css_list: Vec<String>,
    status: approck::server::StatusCode,
    script_list: Vec<String>,
    style_list: Vec<String>,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            title: None,
            head: Vec::new(),
            body: Vec::new(),
            tail: Vec::new(),
            js_list: vec!["/app.js".to_string()],
            css_list: vec!["/app.css".to_string()],
            status: approck::server::StatusCode::OK,
            script_list: Vec::new(),
            style_list: Vec::new(),
        }
    }
}

impl From<Document> for approck::server::response::HTML {
    fn from(layout: Document) -> approck::server::response::HTML {
        let mut rval = approck::server::response::HTML::new(layout.render().into_string());
        rval.status = layout.status;
        rval
    }
}

impl approck::traits::Document for Document {
    fn add_head(&mut self, chunk: maud::Markup) {
        self.head.push(chunk);
    }

    fn add_body(&mut self, chunk: maud::Markup) {
        self.body.push(chunk);
    }

    fn add_tail(&mut self, chunk: maud::Markup) {
        self.tail.push(chunk);
    }

    fn set_title(&mut self, title: &str) {
        self.title = Some(title.to_string());
    }

    fn add_js(&mut self, module: &str) {
        self.js_list.push(module.to_string());
    }

    fn add_css(&mut self, path: &str) {
        self.css_list.push(path.to_string());
    }

    fn set_status(&mut self, status: approck::server::StatusCode) {
        self.status = status;
    }

    fn add_script(&mut self, script: &str) {
        self.script_list.push(script.to_string());
    }

    fn add_style(&mut self, style: &str) {
        self.style_list.push(style.to_string());
    }
}

#[rustfmt::skip]
impl maud::Render for Document {
    fn render(&self) -> maud::Markup {
        
        html! {
            (maud::DOCTYPE)
            html {
                head {
                    meta charset="utf-8";
                    meta name="viewport" content="width=device-width, initial-scale=1";
                    // Bootstrap CSS
                    link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-9ndCyUaIbzAi2FUVXJi0CjmCapSmO7SnpJef0486qhLnuZ2cdeRhO02iuK6FUUVM" crossorigin="anonymous";
                    link href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.4/css/all.min.css" rel="stylesheet";
                    
                    @for css in &self.css_list {
                        link href=(css) rel="stylesheet" {}
                    }

                    @if let Some(title) = &self.title {
                        title { (title) }
                    }  
                    
                    @for chunk in &self.head {
                        (chunk)
                    }

                    @for style in &self.style_list {
                        style { (style) }
                    }
                }
                body {
                    // Content
                    div.container-fluid {
                        header {
                            // Navigation bar
                            nav.navbar.navbar-expand-lg.bg-teal."py-3" data-bs-theme="white" aria-label="Main navigation" {
                                div.container-fluid."m-0" {
                                    a.navbar-brand."ms-4" href="/" {
                                       // h2 { "approck-example" }
                                        img.nav-logo src="/asset/logo.svg" alt="logo" border="0" {}
                                    }
                                    button.navbar-toggler type="button" data-bs-toggle="collapse" data-bs-target="#navbarNavAltMarkup" aria-controls="navbarNavAltMarkup" aria-expanded="false" aria-label="Toggle navigation" {
                                        span.navbar-toggler-icon {}
                                    }
                                    div.collapse.navbar-collapse id="navbarNavAltMarkup" {
                                        ul.navbar-nav."ms-auto"."p4".nav-bar {
                                            li.nav-item {
                                                a.nav-link href="/" { "Home" }
                                            }
                                            li.nav-item {
                                                a.nav-link href="/example/" { "Examples" }
                                            }

                                            li.nav-item {
                                                a.nav-link href="/about/help" { "Documentation" }
                                            }
                                            li.nav-item {
                                                a.nav-link href="/example/name" { "Forms" }
                                            }
                                            li.nav-item {
                                                a.nav-link href="/api/v1/list" { "API list" }
                                            }
                                            li.nav-item {
                                                a.nav-link href="/foo" { "404" }
                                            }
                                        }
                                    }
                                }
                            }
                            nav.navbar.navbar-expand-lg.bg-white.shadow aria-label="Secondary navigation" {
                                div.container {
                                    button.navbar-toggler type="button" data-bs-toggle="collapse" data-bs-target="#navbarNavAltMarkup" aria-controls="navbarNavAltMarkup" aria-expanded="false" aria-label="Toggle navigation" {
                                        span.navbar-toggler-icon {}
                                    }
                                    div.collapse.navbar-collapse id="navbarNavAltMarkup" {
                                        ul.navbar-nav {
                                            li.nav-item {
                                                a.nav-link."nav2" href="/" { "Tab 1" }
                                            }
                                            li.nav-item {
                                                a.nav-link."nav2" href="/about" { "Tab 2" }
                                            }
                                            li.nav-item {
                                                a.nav-link."nav2" href="/about/" { "Tab 3" }
                                            }
                                            li.nav-item {
                                                a.nav-link."nav2" href="/pricing" { "Tab 4" }
                                            }
                                            li.nav-item {
                                                a.nav-link."nav2" href="/api/v1/list" { "Tab 5" }
                                            }
                                            li.nav-item {
                                                a.nav-link."nav2" href="/foo" { "Tab 6" }
                                            }
                                        }
                                    }
                                    div.justify-content-center {
                                        form.d-flex {
                                            input.form-control."me-2" type="search" placeholder="Search" aria-label="Search";
                                            button.btn.btn-outline-success type="submit" { "Search" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    hr;
                    div.container-fluid.main-content {  //Made the main content into a container-fluid. If you create a new page, don't forget to wrap it in a div.container
                        @for chunk in &self.body {
                            (chunk)
                        }
                    }
                }
                // Footer
                footer.footer."p-2".fixed-bottom {
                    div.container.text-start."pt-3" {
                        p.lead {
                            "Footer Here"
                        }
                    }
                }
                // Bootstrap JS
                script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/js/bootstrap.bundle.min.js" integrity="sha384-geWF76RCwLtnZ8qwWowPQNguL3RmwHVBC9FhGdlKrxdiJJigb/j/68SIy3Te4Bkz" crossorigin="anonymous" {}

                @for module in &self.js_list {
                    script src=(module) type="module" {}
                }

                @for script in &self.script_list {
                    script type="module" { (script) }
                }
            }
        }
    }
}
