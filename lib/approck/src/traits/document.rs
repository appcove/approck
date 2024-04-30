use crate::server::response::HTML;
use crate::server::StatusCode;

pub trait Document: Into<HTML> {
    fn add_head(&mut self, chunk: maud::Markup);
    fn add_body(&mut self, chunk: maud::Markup);
    fn add_tail(&mut self, chunk: maud::Markup);
    fn set_title(&mut self, title: &str);
    fn add_js(&mut self, path: &str);
    fn add_css(&mut self, path: &str);
    fn add_script(&mut self, script: &str);
    fn add_style(&mut self, style: &str);
    fn set_status(&mut self, status: StatusCode);
}

//LUKE: ideas on how we can indicate what kind of document we want?  Admin/public/user/print/plain ?
pub trait DocumentModule {
    fn get_document(&self) -> impl Document;
}
