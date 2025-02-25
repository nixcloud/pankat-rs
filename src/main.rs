mod articles;
mod auth;
mod config;
mod db;
mod error;
mod file_monitor;
mod handlers;
mod registry;
mod renderer;

use axum::{
    routing::{get, post},
    Router,
};

use clap::{Arg, ArgAction, Command};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use tokio::signal;
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt::init();
    let matches = Command::new("pankat")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Joachim Schiele <js@lastlog.de>")
        .about("https://github.com/nixcloud/pankat - static site generator")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("PATH")
                .help("Absolute path where the media/*.jpg and posts/*.md files of your blog are located")
                .required(true)
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("PATH")
                .help("Absolute path, where pankat 'maintains' the generated html files by adding/deleting/updating them")
                .required(true)
        )
        .arg(
            Arg::new("assets")
                .short('a')
                .long("assets")
                .value_name("PATH")
                .help("An absolute assets path, where js/wasm/css/templates/lua/... files are stored")
                .required(true)
        )
        .arg(
            Arg::new("database")
                .short('d')
                .long("database")
                .value_name("PATH")
                .help("An absolute path where 'only' the database is stored (don't put this into output!)")
                .required(true)
        )
        .arg(
            Arg::new("brand")
                .short('b')
                .long("brand")
                .value_name("URL")
                .help("A brand name shown on every page top left")
                .required(false)
                .default_value("lastlog.de/blog")
        )
        .arg(
            Arg::new("static")
                .short('s')
                .long("static")
                .help("Only build documents and exit (static blog generator)")
                .required(false)
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Port number where pankat listens for incoming connections for browser connections")
                .default_value("5000"),
        )
        .get_matches();

    let config = config::Config::new(
        std::path::Path::new(matches.get_one::<String>("input").unwrap()).into(),
        std::path::Path::new(matches.get_one::<String>("output").unwrap()).into(),
        std::path::Path::new(matches.get_one::<String>("assets").unwrap()).into(),
        std::path::Path::new(matches.get_one::<String>("database").unwrap()).into(),
        matches.get_one::<String>("port").unwrap().parse().unwrap(),
        matches.get_one::<String>("brand").unwrap().parse().unwrap(),
        matches.get_flag("static"),
    );
    config::Config::initialize(config).expect("Failed to initialize config");
    let cfg = config::Config::get();

    println!("-------------------------------------------------");
    println!("Input Path: {}", cfg.input.display());
    println!("Output Path: {}", cfg.output.display());
    println!("Assets Path: {}", cfg.assets.display());
    println!("Database Path: {}", cfg.database.display());
    println!("Port Number: {}", cfg.port);
    println!("Brand: {}", cfg.brand);
    println!("Static build only: {}", cfg.static_build_only);
    println!("-------------------------------------------------");

    // Initialize SQLite database with Diesel
    let pool = db::establish_connection_pool();

    articles::collect_garbage(&pool);
    articles::scan_articles(&pool);
    articles::build_articles(&pool);

    if cfg.static_build_only {
        println!("Static build only, exiting...");
        return Ok(());
    }

    // Setup broadcast channel for shutdown coordination
    let (shutdown_tx, _) = broadcast::channel::<()>(1);

    // FIXME adapt this concept for with_state
    // struct AppState {
    //     pool: Pool<ConnectionManager<SqliteConnection>>,
    // }
    // https://docs.rs/axum/latest/axum/#sharing-state-with-handlers

    // Initialize file monitoring
    let monitor_handle =
        file_monitor::spawn_async_monitor(pool.clone(), cfg.input.clone(), shutdown_tx.subscribe())
            .map_err(|e| Box::<dyn std::error::Error + Send + Sync>::from(e))?;

    // Create router
    let app = Router::new()
        .route("/posts/*path", get(handlers::serve_input))
        .route("/media/*path", get(handlers::serve_input))
        .route("/assets/*path", get(handlers::serve_assets))
        .route("/api/auth/register", post(handlers::register))
        .route("/api/auth/login", post(handlers::login))
        .route("/api/protected", get(handlers::protected))
        .route("/api/ws", get(handlers::websocket_route))
        .route("/", get(handlers::serve_output))
        .route("/*path", get(handlers::serve_output))
        .layer(CorsLayer::permissive())
        .with_state(pool.clone());

    // Start server
    let address_config = format!("[::]:{}", cfg.port);
    let addr = address_config.parse::<std::net::SocketAddr>().unwrap();

    // Create a listener with retry logic
    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            println!("Listening on: {}", address_config);
            listener
        }
        Err(e) => {
            eprintln!("Failed to bind to port {}: {}", cfg.port, e);
            // If port is in use, try to clean up and exit
            println!("Initiating cleanup sequence...");
            if let Err(send_err) = shutdown_tx.send(()) {
                eprintln!("Error broadcasting shutdown signal: {}", send_err);
            }
            // Wait for monitor to cleanup
            if let Err(e) = monitor_handle.await {
                eprintln!("Error during monitor shutdown: {}", e);
            }
            return Err(Box::<dyn std::error::Error + Send + Sync>::from(e));
        }
    };

    // Start server with graceful shutdown
    println!("Press Ctrl+C to stop the server...");
    tokio::select! {
        result = axum::serve(listener, app) => {
            if let Err(e) = result {
                eprintln!("Server error: {}", e);
                // Initiate cleanup before returning error
                if let Err(send_err) = shutdown_tx.send(()) {
                    eprintln!("Error broadcasting shutdown signal: {}", send_err);
                }
            }
        }
        _ = signal::ctrl_c() => {
            println!("\nReceived Ctrl+C, initiating graceful shutdown...");
            // Broadcast shutdown signal to all tasks
            if let Err(e) = shutdown_tx.send(()) {
                eprintln!("Error broadcasting shutdown signal: {}", e);
            }
        }
    }

    // Wait for the file monitor to complete its cleanup
    println!("Waiting for file monitor to complete shutdown...");
    if let Err(e) = monitor_handle.await {
        eprintln!("Error during monitor shutdown: {}", e);
        return Err(Box::<dyn std::error::Error + Send + Sync>::from(e));
    }
    println!("Graceful shutdown complete");

    Ok(())
}
