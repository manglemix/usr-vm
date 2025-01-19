use std::{
    backtrace::Backtrace, io::{LineWriter, Write}, net::SocketAddr, panic::set_hook, path::Path, sync::atomic::AtomicBool
};

use axum::{routing::get, Router};
use discord_webhook2::webhook::DiscordWebhook;
use parking_lot::Mutex;
use rustls::crypto::ring::default_provider;
use sea_orm::{Database, DatabaseConnection};
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_http::cors::Any;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;
use webhook::BatchedWebhook;

mod scheduler;
mod manifest;
mod webhook;
mod backup;

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
    new_orders_webhook: Option<String>,
    order_updates_webhook: Option<String>,
}

struct UsrState {
    db: DatabaseConnection,
    new_orders_webhook: Option<BatchedWebhook>,
    order_updates_webhook: Option<BatchedWebhook>,
    backup_task_running: AtomicBool
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

    set_hook(Box::new(|info| {
        let backtrace = Backtrace::capture();
        error!("{}\n{backtrace}", info);
    }));

    let db = Database::connect("sqlite://usr-db.sqlite?mode=rwc").await?;
    let config: Config = serde_json::from_reader(std::fs::File::open("config.json")?)?;

    if Path::new(".reset-db").exists() {
        info!("Resetting DB");
        let directive = std::fs::read_to_string(".reset-db")?;

        match directive.as_str() {
            "scheduler" => {
                scheduler::reset_tables(&db).await?;
                info!("Reset scheduler tables");
            }
            "manifest" => {
                manifest::reset_tables(&db).await?;
                info!("Reset manifest tables");
            }
            "all" => {
                scheduler::reset_tables(&db).await?;
                manifest::reset_tables(&db).await?;
                info!("Reset all tables");
            }
            _ => {
                error!("Invalid directive in .reset-db");
                return Ok(());
            }
        }

        std::fs::remove_file(".reset-db")?;
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
                .layer({
                    let mut layer = tower_http::cors::CorsLayer::new();
                    #[cfg(debug_assertions)]
                    {
                        layer = layer.allow_origin(Any);
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        layer = layer.allow_origin(
                            "https://utahrobotics.github.io".parse::<axum::http::HeaderValue>().unwrap(),
                        );
                    }
                    layer
                        .allow_headers(Any)
                        .allow_methods(Any)
                })
                .layer(tower_http::compression::CompressionLayer::new())
        )
        .with_state(Box::leak(Box::new(UsrState {
            db,
            new_orders_webhook: {
                if let Some(new_orders_webhook) = config.new_orders_webhook {
                    Some(DiscordWebhook::new(new_orders_webhook)?.into())
                } else {
                    None
                }
            },
            order_updates_webhook: {
                if let Some(order_updates_webhook) = config.order_updates_webhook {
                    Some(DiscordWebhook::new(order_updates_webhook)?.into())
                } else {
                    None
                }
            },
            backup_task_running: AtomicBool::new(false),
        })));

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
