use std::{
    io::{LineWriter, Write}, net::SocketAddr, path::Path, sync::Arc
};

use axum::{http::HeaderValue, routing::get, Router};
use discord_webhook2::webhook::DiscordWebhook;
use parking_lot::Mutex;
use rustls::crypto::ring::default_provider;
use sea_orm::{Database, DatabaseConnection};
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_http::cors::Any;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

mod scheduler;
mod manifest;

struct LogWriter {
    inner: &'static Mutex<LineWriter<std::fs::File>>,
}

impl Write for LogWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.lock().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.lock().flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.inner.lock().write_all(buf)
    }
}

#[derive(Deserialize)]
struct Config {
    new_orders_webhook: String,
    order_updates_webhook: String,
}

struct UsrState {
    db: DatabaseConnection,
    new_orders_webhook: DiscordWebhook,
    order_updates_webhook: DiscordWebhook,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let log_file = Mutex::new(LineWriter::new(std::fs::File::create("usr-backend.log")?));
    let log_file: &_ = Box::leak(Box::new(log_file));

    FmtSubscriber::builder()
        .with_file(true)
        .with_level(true)
        .with_line_number(true)
        .with_target(true)
        .with_thread_names(true)
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339())
        .pretty()
        .with_ansi(false)
        .with_writer(|| {
            let mut lock = log_file.lock();

            if lock.get_ref().metadata().is_err() {
                println!("Error");
                if let Ok(file) = std::fs::File::create("usr-backend.log") {
                    *lock.get_mut() = file;
                }
            }

            LogWriter { inner: log_file }
        })
        .init();

    let db = Database::connect("sqlite://usr-db.sqlite?mode=rwc").await?;
    let config: Config = serde_json::from_reader(std::fs::File::open("config.json")?)?;

    if Path::new(".reset-db").exists() {
        info!("Resetting DB");
        std::fs::remove_file(".reset-db")?;
        scheduler::reset_tables(&db).await?;
        manifest::reset_tables(&db).await?;
        info!("DB Reset");
    }

    let app = Router::new()
        .route(
            "/",
            get(|| async { format!("Version: {}", env!("CARGO_PKG_VERSION")) }),
        )
        .nest(
            "/api",
            Router::new()
                .nest("/scheduler", scheduler::router())
                .nest("/manifest", manifest::router()),
        )
        .layer(
            ServiceBuilder::new()
                .layer(
                    tower_http::cors::CorsLayer::new()
                        .allow_origin([
                            "https://utahrobotics.github.io".parse::<HeaderValue>().unwrap(),
                            #[cfg(debug_assertions)]
                            "http://127.0.0.1:5173".parse::<HeaderValue>().unwrap(),
                        ])
                        .allow_methods(Any),
                )
                .layer(tower_http::compression::CompressionLayer::new())
        )
        .with_state(Arc::new(UsrState {
            db,
            new_orders_webhook: DiscordWebhook::new(config.new_orders_webhook)?,
            order_updates_webhook: DiscordWebhook::new(config.order_updates_webhook)?,
        }));

    default_provider()
        .install_default()
        .map_err(|_| anyhow::anyhow!("Failed to install ring CryptoProvider"))?;

    info!("Starting server");
    #[cfg(not(debug_assertions))]
    {
        use axum_server::tls_rustls::RustlsConfig;
        let config = RustlsConfig::from_pem_file("cert.pem", "key.pem").await?;
        let addr = SocketAddr::from(([0, 0, 0, 0], 443));
        axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service())
            .await
            .map_err(Into::into)
    }
    #[cfg(debug_assertions)]
    {
        let addr = SocketAddr::from(([0, 0, 0, 0], 80));
        axum_server::bind(addr)
            .serve(app.into_make_service())
            .await
            .map_err(Into::into)
    }
}
