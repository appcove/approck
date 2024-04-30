use approck::server::WebServerModule;
use approck_example::{AppConfig, AppSystem};

#[tokio::main]
async fn main() {
    // get path to this executable
    let toml_path = std::env::current_exe()
        .expect("current_exe")
        .with_extension("toml");
    let toml_data = std::fs::read_to_string(&toml_path).expect("read_to_string");
    let toml_conf: AppConfig = match toml::from_str(&toml_data) {
        Ok(toml_conf) => toml_conf,
        Err(e) => {
            println!("Configuration Parsing Error: {:#?}", e);
            std::process::exit(1);
        }
    };

    let app: &'static AppSystem =
        Box::leak(Box::new(toml_conf.into_system().await.expect("app system")));

    println!("Welcome to `{}`", env!("CARGO_PKG_NAME"));
    println!(
        "  Visit https://local.acp7.net:{}/ in your browser",
        app.webserver_system().port()
    );

    approck::server::serve(app).await
}
