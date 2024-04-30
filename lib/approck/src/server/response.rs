/*

pub struct ReturnTypes {
    pub Bytes: bool,
    pub Text: bool,
    pub Empty: bool,
    pub HTML: bool,
    pub JavaScript: bool,
    pub CSS: bool,
    pub JSON: bool,
    pub SVG: bool,
    pub NotFound: bool,
    pub Redirect: bool,
    pub WebSocketUpgrade: bool,
}

*/

use futures::TryStreamExt;
use headers::HeaderMapExt;

pub type Result = granite::Result<Response>;

#[derive(Default)]
pub struct Bytes {
    pub content: bytes::Bytes,
    pub status: http::StatusCode,
    pub headers: http::HeaderMap,
}

impl Bytes {
    pub fn new(content: impl Into<bytes::Bytes>) -> Self {
        Self {
            content: content.into(),
            ..Self::default()
        }
    }
}

impl salvo_core::Scribe for Bytes {
    fn render(self, res: &mut salvo_core::prelude::Response) {
        res.headers
            .typed_insert(headers::ContentType::from(mime::APPLICATION_OCTET_STREAM));
        res.headers.extend(self.headers);
        res.status_code = Some(self.status);
        res.body(self.content.into());
    }
}

pub struct Stream {
    /// NOTE: This field is intentonally private to hide the implementation details of
    /// [`BytesContent::Stream`].
    content: sync_wrapper::SyncWrapper<
        futures::stream::BoxStream<
            'static,
            std::result::Result<salvo_core::http::body::BytesFrame, salvo_core::BoxedError>,
        >,
    >,
    pub status: http::StatusCode,
    pub headers: http::HeaderMap,
}

impl Stream {
    pub fn new<S>(stream: S) -> Self
    where
        S: futures::Stream<Item = granite::Result<crate::server::Frame>> + Send + 'static,
    {
        let mapped: futures::stream::BoxStream<_> = Box::pin(
            stream
                .map_ok::<salvo_core::http::body::BytesFrame, _>(|frame| match frame {
                    crate::server::Frame::Data(data) => data.into(),
                    crate::server::Frame::Trailers(trailers) => salvo_core::http::body::BytesFrame(
                        salvo_core::http::body::Frame::trailers(trailers),
                    ),
                })
                .map_err::<salvo_core::BoxedError, _>(|error| Box::new(error.into_std())),
        );
        let content = sync_wrapper::SyncWrapper::new(mapped);
        Self {
            content,
            status: http::StatusCode::default(),
            headers: http::HeaderMap::default(),
        }
    }
}

impl<S, F> From<S> for Stream
where
    S: futures::Stream<Item = granite::Result<F>> + Send + 'static,
    F: Into<crate::server::Frame>,
{
    fn from(stream: S) -> Self {
        Self::new(stream.map_ok(|into_frame| into_frame.into()))
    }
}

impl salvo_core::Scribe for Stream {
    fn render(self, res: &mut salvo_core::prelude::Response) {
        res.headers
            .typed_insert(headers::ContentType::from(mime::APPLICATION_OCTET_STREAM));
        res.headers.extend(self.headers);
        res.status_code = Some(self.status);
        res.body(salvo_core::http::ResBody::Stream(self.content));
    }
}

pub struct Text {
    pub content: String,
    pub status: http::StatusCode,
    pub headers: http::HeaderMap,
}

impl Text {
    pub fn new(content: String) -> Self {
        Self {
            content,
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
        }
    }
}

impl salvo_core::Scribe for Text {
    fn render(self, res: &mut salvo_core::prelude::Response) {
        res.headers
            .typed_insert(headers::ContentType::from(mime::TEXT_PLAIN_UTF_8));
        res.headers.extend(self.headers);
        res.status_code = Some(self.status);
        res.body(self.content.into());
    }
}

impl From<&str> for Text {
    fn from(content: &str) -> Self {
        Self::new(content.to_string())
    }
}

impl From<String> for Text {
    fn from(content: String) -> Self {
        Self::new(content)
    }
}

pub struct Empty {
    pub status: http::StatusCode,
    pub headers: http::HeaderMap,
}

impl salvo_core::Scribe for Empty {
    fn render(self, res: &mut salvo_core::prelude::Response) {
        res.headers.extend(self.headers);
        res.status_code = Some(self.status);
    }
}

impl Default for Empty {
    fn default() -> Self {
        Self {
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct HTML {
    pub content: String,
    pub status: http::StatusCode,
    pub headers: http::HeaderMap,
}

impl HTML {
    pub fn new(content: String) -> Self {
        Self {
            content,
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
        }
    }
}

impl salvo_core::Scribe for HTML {
    fn render(self, res: &mut salvo_core::prelude::Response) {
        res.headers
            .typed_insert(headers::ContentType::from(mime::TEXT_HTML_UTF_8));
        res.headers.extend(self.headers);
        res.status_code = Some(self.status);
        res.body(self.content.into());
    }
}

impl From<&str> for HTML {
    fn from(content: &str) -> Self {
        Self::new(content.to_string())
    }
}

impl From<String> for HTML {
    fn from(content: String) -> Self {
        Self::new(content)
    }
}

pub struct JavaScript {
    pub content: String,
    pub status: http::StatusCode,
    pub headers: http::HeaderMap,
}

impl JavaScript {
    pub fn new(content: String) -> Self {
        Self {
            content,
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
        }
    }
}

impl salvo_core::Scribe for JavaScript {
    fn render(self, res: &mut salvo_core::prelude::Response) {
        res.headers
            .typed_insert(headers::ContentType::from(mime::TEXT_JAVASCRIPT));
        res.headers.extend(self.headers);
        res.status_code = Some(self.status);
        res.body(self.content.into());
    }
}

impl From<&str> for JavaScript {
    fn from(content: &str) -> Self {
        Self::new(content.to_string())
    }
}

pub struct CSS {
    pub content: String,
    pub status: http::StatusCode,
    pub headers: http::HeaderMap,
}

impl CSS {
    pub fn new(content: String) -> Self {
        Self {
            content,
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
        }
    }
}

impl salvo_core::Scribe for CSS {
    fn render(self, res: &mut salvo_core::prelude::Response) {
        res.headers
            .typed_insert(headers::ContentType::from(mime::TEXT_CSS));
        res.headers.extend(self.headers);
        res.status_code = Some(self.status);
        res.body(self.content.into());
    }
}

impl From<&str> for CSS {
    fn from(content: &str) -> Self {
        Self::new(content.to_string())
    }
}

pub struct JSON {
    pub content: String,
    pub status: http::StatusCode,
    pub headers: http::HeaderMap,
}

impl JSON {
    pub fn new(content: String) -> Self {
        Self {
            content,
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
        }
    }
}

impl salvo_core::Scribe for JSON {
    fn render(self, res: &mut salvo_core::prelude::Response) {
        res.headers
            .typed_insert(headers::ContentType::from(mime::APPLICATION_JSON));
        res.headers.extend(self.headers);
        res.status_code = Some(self.status);
        res.body(self.content.into());
    }
}

impl From<&str> for JSON {
    fn from(content: &str) -> Self {
        Self::new(content.to_string())
    }
}

impl From<String> for JSON {
    fn from(content: String) -> Self {
        Self::new(content)
    }
}

impl From<serde_json::Value> for JSON {
    fn from(content: serde_json::Value) -> Self {
        Self::new(content.to_string())
    }
}

pub struct SVG {
    pub content: String,
    pub status: http::StatusCode,
    pub headers: http::HeaderMap,
}

impl SVG {
    pub fn new(content: String) -> Self {
        Self {
            content,
            status: http::StatusCode::OK,
            headers: http::HeaderMap::new(),
        }
    }
}

impl salvo_core::Scribe for SVG {
    fn render(self, res: &mut salvo_core::prelude::Response) {
        res.headers
            .typed_insert(headers::ContentType::from(mime::IMAGE_SVG));
        res.headers.extend(self.headers);
        res.status_code = Some(self.status);
        res.body(self.content.into());
    }
}

impl From<&str> for SVG {
    fn from(content: &str) -> Self {
        Self::new(content.to_string())
    }
}

pub struct NotFound;

impl Default for NotFound {
    fn default() -> Self {
        Self
    }
}

impl salvo_core::Scribe for NotFound {
    fn render(self, res: &mut salvo_core::prelude::Response) {
        res.headers
            .typed_insert(headers::ContentType::from(mime::TEXT_PLAIN_UTF_8));
        res.status_code = Some(http::StatusCode::NOT_FOUND);
        res.body("Not Found".into());
    }
}

#[derive(Debug)]
pub struct Redirect {
    pub location: String,
    pub status: http::StatusCode,
}

impl Redirect {
    pub fn see_other(location: String) -> Self {
        Self {
            location,
            status: http::StatusCode::SEE_OTHER,
        }
    }

    pub fn temporary(location: String) -> Self {
        Self {
            location,
            status: http::StatusCode::TEMPORARY_REDIRECT,
        }
    }

    pub fn permanent(location: String) -> Self {
        Self {
            location,
            status: http::StatusCode::PERMANENT_REDIRECT,
        }
    }
}

impl salvo_core::Scribe for Redirect {
    fn render(self, res: &mut salvo_core::prelude::Response) {
        res.headers.insert(
            http::header::LOCATION,
            http::header::HeaderValue::from_str(&self.location).unwrap(),
        );
        res.status_code = Some(self.status);
    }
}

impl From<&str> for Redirect {
    fn from(location: &str) -> Self {
        Self::see_other(location.to_string())
    }
}

pub struct WebSocketUpgrade(pub(crate) salvo_core::Response);

impl salvo_core::Scribe for WebSocketUpgrade {
    fn render(self, res: &mut salvo_core::prelude::Response) {
        *res = self.0
    }
}

pub enum Response {
    Bytes(Bytes),
    Stream(Stream),
    Text(Text),
    Empty(Empty),
    HTML(HTML),
    JavaScript(JavaScript),
    CSS(CSS),
    JSON(JSON),
    SVG(SVG),
    NotFound(NotFound),
    Redirect(Redirect),
    WebSocketUpgrade(WebSocketUpgrade),
}

impl salvo_core::Scribe for Response {
    fn render(self, res: &mut salvo_core::prelude::Response) {
        match self {
            Response::Bytes(bytes) => res.render(bytes),
            Response::Stream(stream) => res.render(stream),
            Response::Text(text) => res.render(text),
            Response::Empty(empty) => res.render(empty),
            Response::HTML(html) => res.render(html),
            Response::JavaScript(javascript) => res.render(javascript),
            Response::CSS(css) => res.render(css),
            Response::JSON(json) => res.render(json),
            Response::SVG(svg) => res.render(svg),
            Response::NotFound(not_found) => res.render(not_found),
            Response::Redirect(redirect) => res.render(redirect),
            Response::WebSocketUpgrade(websocket_upgrade) => res.render(websocket_upgrade),
        }
    }
}
