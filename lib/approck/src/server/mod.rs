use granite::ErrorKind;
use headers::HeaderMapExt;
pub use http::StatusCode;

pub mod exports;
pub mod response;
pub mod websocket;

use http::{header::UPGRADE, HeaderMap};
use salvo_core::{conn::rustls, Listener};
use std::net::IpAddr;

#[derive(Debug, serde::Deserialize)]
pub struct WebServerConfig {
    #[serde(default = "default_host")]
    pub host: IpAddr,

    pub port: u16,
    #[serde(default = "default_tls_cert_pem")]
    pub tls_cert_pem: String,

    #[serde(default = "default_tls_key_pem")]
    pub tls_key_pem: String,
}

fn default_host() -> IpAddr {
    "127.0.0.1".parse().unwrap()
}

fn default_tls_cert_pem() -> String {
    include_str!("../../../../meta/localhost.crt.pem").to_string()
}

fn default_tls_key_pem() -> String {
    include_str!("../../../../meta/localhost.key.pem").to_string()
}

impl WebServerConfig {
    pub fn into_system(self) -> WebServerSystem {
        WebServerSystem { config: self }
    }
}

pub struct WebServerSystem {
    config: WebServerConfig,
}

impl WebServerSystem {
    pub fn host(&self) -> IpAddr {
        self.config.host
    }
    pub fn port(&self) -> u16 {
        self.config.port
    }
    pub fn tls_cert_pem(&self) -> &str {
        &self.config.tls_cert_pem
    }
    pub fn tls_key_pem(&self) -> &str {
        &self.config.tls_key_pem
    }
}

pub trait WebServerModule {
    fn webserver_system(&self) -> &WebServerSystem;

    #[allow(async_fn_in_trait)]
    fn webserver_route<'a>(
        &'static self,
        req: Request<'a>,
    ) -> impl std::future::Future<Output = crate::server::response::Result> + Send;

    /// Provide the user with some nice looking response.
    fn webserver_handle_error(&self, error: granite::Error) -> crate::server::response::Result {
        Ok(standard_handle_error(error))
    }
}

#[derive(Debug)]
pub struct Request<'a>(&'a mut salvo_core::Request);

impl<'a> From<&'a mut salvo_core::Request> for Request<'a> {
    fn from(request: &'a mut salvo_core::Request) -> Self {
        Self(request)
    }
}

#[non_exhaustive]
pub enum Frame {
    Data(bytes::Bytes),
    Trailers(http::HeaderMap),
}

impl From<Vec<u8>> for Frame {
    fn from(value: Vec<u8>) -> Self {
        Self::Data(value.into())
    }
}

impl From<&'static [u8]> for Frame {
    fn from(value: &'static [u8]) -> Self {
        Self::Data(value.into())
    }
}

impl From<bytes::Bytes> for Frame {
    fn from(value: bytes::Bytes) -> Self {
        Self::Data(value)
    }
}

impl From<HeaderMap> for Frame {
    fn from(value: HeaderMap) -> Self {
        Self::Trailers(value)
    }
}

impl<'a> Request<'a> {
    /// If the request is a websocket upgrade request, will return a websocket upgrade response and
    /// run the `websocket_handler` with the created socket, otherwise this method will return
    /// `None`.
    pub async fn upgrade_to_websocket<HANDLER, FUT>(
        &mut self,
        websocket_handler: HANDLER,
    ) -> granite::Result<Option<crate::server::response::WebSocketUpgrade>>
    where
        HANDLER: Fn(websocket::WebSocket) -> FUT + Send + 'static,
        FUT: std::future::Future<Output = ()> + Send + 'static,
    {
        if !self.is_upgrade() {
            return Ok(None);
        }

        websocket::upgrade(self.0, websocket_handler)
            .await
            .map(Some)
    }

    /// Get the chunks of the path as a vec.  For example:  
    ///   `/a/b/c/d` -> `["a", "b", "c", "d"]`
    pub fn path_chunks(&self) -> Vec<&str> {
        self.0.uri().path().split('/').skip(1).collect()
    }

    pub fn path(&self) -> &str {
        self.0.uri().path()
    }

    pub fn uri_string(&self) -> String {
        self.0.uri().to_string()
    }

    /// Get the http::HeaderMap from the request
    pub fn headers(&self) -> &http::HeaderMap {
        self.0.headers()
    }

    pub fn session_token(&self) -> String {
        if let Some(cookie) = self.0.cookie("SessionToken") {
            let cookie = cookie.value_trimmed();
            if cookie.len() == 64 {
                return cookie.to_owned();
            }
        }

        granite::ts_random_hex(64)
    }

    pub fn iter_query_pairs(&self) -> url::form_urlencoded::Parse<'_> {
        url::form_urlencoded::parse(self.0.uri().query().unwrap_or("").as_bytes())
    }

    pub async fn read_body_as_bytes(&mut self) -> Result<Vec<u8>, granite::Error> {
        match self.0.body() {
            salvo_core::http::ReqBody::Once(bytes) => Ok(bytes.to_vec()),
            salvo_core::http::ReqBody::None => Ok(Vec::new()),
            _ => Err(granite::Error::new(ErrorKind::Unexpected)
                .add_context("Unable to read request body as bytes")),
        }
    }

    pub fn is_upgrade(&self) -> bool {
        let connection = match self.0.headers().typed_get::<headers::Connection>() {
            Some(connection) => connection,
            None => return false,
        };
        let upgrade = match self.0.headers().typed_get::<headers::Upgrade>() {
            Some(upgrade) => upgrade,
            None => return false,
        };
        connection.contains(UPGRADE) && upgrade == headers::Upgrade::websocket()
    }

    // write a function to iter_body_query_pairs
    pub async fn read_body_query_pairs(&mut self) -> Vec<(String, String)> {
        let bytes = self.read_body_as_bytes().await.unwrap();
        let rval: Vec<_> = url::form_urlencoded::parse(&bytes)
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();
        rval
    }

    pub fn has_query_string(&self) -> bool {
        self.0.uri().query().is_some()
    }

    pub fn is_post(&self) -> bool {
        self.0.method() == http::Method::POST
    }

    pub fn is_get(&self) -> bool {
        self.0.method() == http::Method::GET
    }
}

// We would like to have different traits that APP implements, different projects might implement
// different sets of traits. How do we make these traits available in the application code after
// passing through here?
pub async fn serve<APP>(app: &'static APP)
where
    APP: WebServerModule + Send + Sync,
{
    let webserver_system = &app.webserver_system();

    let addr = std::net::SocketAddr::from((webserver_system.host(), webserver_system.port()));

    // Do not use client certificate authentication.
    let tls_config = rustls::RustlsConfig::new(
        rustls::Keycert::new()
            .key(webserver_system.tls_key_pem().to_owned())
            .cert(webserver_system.tls_cert_pem().to_owned()),
    );

    let tcp_listener = salvo_core::conn::TcpListener::new(addr).rustls(tls_config.clone());
    let quic_listener = salvo_core::conn::QuinnListener::new(tls_config, addr);
    let acceptor = quic_listener.join(tcp_listener).try_bind().await.unwrap();

    let handler = MyHandler {
        app,
        port: webserver_system.port(),
    };
    let router = salvo_core::Router::new().path("<**>").goal(handler);

    salvo_core::server::Server::new(acceptor)
        .try_serve(router)
        .await
        .unwrap();
}

/// Basic rendering of errors.
pub fn standard_handle_error(error: granite::Error) -> crate::server::response::Response {
    eprintln!("{error:#?}");
    response::Response::Empty(response::Empty {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        headers: HeaderMap::default(),
    })
}

struct MyHandler<APP: 'static> {
    app: &'static APP,
    port: u16,
}

#[async_trait::async_trait]
impl<APP> salvo_core::handler::Handler for MyHandler<APP>
where
    APP: WebServerModule + Send + Sync + 'static,
{
    async fn handle(
        &self,
        salvo_request: &mut salvo_core::Request,
        _depot: &mut salvo_core::Depot,
        salvo_response: &mut salvo_core::Response,
        _flow_control: &mut salvo_core::FlowCtrl,
    ) {
        let request = Request::from(salvo_request);
        let session_token = request.session_token();

        let port = self.port;
        let path = request.path().to_owned();

        let response = match self.app.webserver_route(request).await {
            Ok(response) => response,
            Err(error) => self
                .app
                .webserver_handle_error(error)
                .unwrap_or_else(standard_handle_error),
        };
        salvo_response.render(response);

        // if the request ends in .js, then add cache headers for 30 seconds
        if path.ends_with(".js") || path.ends_with(".css") {
            salvo_response.headers.insert(
                "Cache-Control",
                "public, max-age=30, immutable".parse().unwrap(),
            );
        }

        // additional header to let browsers know that they can use QUIC/HTTP3
        // Note: this must be added after .render() or it won't be included.
        salvo_response.headers.insert(
            "alt-svc",
            format!(r#"h3=":{port}"; ma=2592000"#).parse().unwrap(),
        );

        // Add session_token cookie
        salvo_response.add_cookie(
            cookie::Cookie::build(("SessionToken", session_token))
                .http_only(true)
                .max_age(cookie::time::Duration::days(3650))
                .path("/")
                .into(),
        );
    }
}
