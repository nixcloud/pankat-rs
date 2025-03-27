mod articles;
mod auth;
mod config;
mod db;
mod error;
mod file_monitor;
mod handlers;
mod registry;
mod renderer;
use crate::config::*;
use crate::renderer::pandoc::check_pandoc;
use axum::{
    routing::{get, post},
    Router,
};
use clap::{Arg, ArgAction, Command};
use colored::Colorize;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use std::collections::HashMap;
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
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("PATH")
                .help("Absolute path, where pankat 'maintains' the generated html files by adding/deleting/updating them")
                .default_value("documents/output")
        )
        .arg(
            Arg::new("assets")
                .short('a')
                .long("assets")
                .value_name("PATH")
                .help("An absolute assets path, where js/wasm/css/templates/lua/... files are stored")
                .default_value("documents/assets")
        )
        .arg(
            Arg::new("wasm")
                .short('w')
                .long("wasm")
                .help("The bundled pankat-wasm executable built from rust")
                .value_name("PATH")
                .default_value("documents/wasm")
        )
        .arg(
            Arg::new("database")
                .short('d')
                .long("database")
                .value_name("PATH")
                .help("An absolute path where 'only' the database is stored (don't put this into output!)")
                .default_value("documents")
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
            Arg::new("jwt_token")
                .short('j')
                .long("jwt_token")
                .value_name("STRING")
                .help("A JWT-Token used for client authentication/authentification")
                .required(false)
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
            Arg::new("flat")
                .short('f')
                .long("flat")
                .help("Flatten the output directory like (foo/bar.mdwn -> bar.html)")
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

    let mut config_values: HashMap<String, ConfigValue> = HashMap::new();

    config_values.insert(
        "input".to_string(),
        ConfigValue {
            value: ConfigValueType::Path(
                matches
                    .get_one::<String>("input")
                    .map(|v| std::path::Path::new(v).into()),
            ),
            is_default: Some(clap::parser::ValueSource::DefaultValue)
                == matches.value_source("input"),
        },
    );

    config_values.insert(
        "output".to_string(),
        ConfigValue {
            value: ConfigValueType::Path(
                matches
                    .get_one::<String>("output")
                    .map(|v| std::path::Path::new(v).into()),
            ),
            is_default: Some(clap::parser::ValueSource::DefaultValue)
                == matches.value_source("output"),
        },
    );

    config_values.insert(
        "assets".to_string(),
        ConfigValue {
            value: ConfigValueType::Path(
                matches
                    .get_one::<String>("assets")
                    .map(|v| std::path::Path::new(v).into()),
            ),
            is_default: Some(clap::parser::ValueSource::DefaultValue)
                == matches.value_source("assets"),
        },
    );

    config_values.insert(
        "wasm".to_string(),
        ConfigValue {
            value: ConfigValueType::Path(
                matches
                    .get_one::<String>("wasm")
                    .map(|v| std::path::Path::new(v).into()),
            ),
            is_default: Some(clap::parser::ValueSource::DefaultValue)
                == matches.value_source("wasm"),
        },
    );

    config_values.insert(
        "database".to_string(),
        ConfigValue {
            value: ConfigValueType::Path(
                matches
                    .get_one::<String>("database")
                    .map(|v| std::path::Path::new(v).into()),
            ),
            is_default: Some(clap::parser::ValueSource::DefaultValue)
                == matches.value_source("database"),
        },
    );

    config_values.insert(
        "brand".to_string(),
        ConfigValue {
            value: ConfigValueType::String(matches.get_one::<String>("brand").map(|v| v.into())),
            is_default: Some(clap::parser::ValueSource::DefaultValue)
                == matches.value_source("brand"),
        },
    );

    config_values.insert(
        "jwt_token".to_string(),
        ConfigValue {
            value: ConfigValueType::String(
                matches.get_one::<String>("jwt_token").map(|v| v.into()),
            ),
            is_default: Some(clap::parser::ValueSource::DefaultValue)
                == matches.value_source("jwt_token"),
        },
    );

    config_values.insert(
        "port".to_string(),
        ConfigValue {
            value: ConfigValueType::Number(
                matches
                    .get_one::<String>("port")
                    .map(|port| port.parse::<u16>().unwrap()),
            ),
            is_default: Some(clap::parser::ValueSource::DefaultValue)
                == matches.value_source("port"),
        },
    );

    config_values.insert(
        "static".to_string(),
        ConfigValue {
            value: ConfigValueType::Bool(matches.get_one::<bool>("static").copied()),
            is_default: Some(clap::parser::ValueSource::DefaultValue)
                == matches.value_source("static"),
        },
    );

    config_values.insert(
        "flat".to_string(),
        ConfigValue {
            value: ConfigValueType::Bool(matches.get_one::<bool>("flat").copied()),
            is_default: Some(clap::parser::ValueSource::DefaultValue)
                == matches.value_source("flat"),
        },
    );

    let config = config::Config::new(config_values);

    config::Config::initialize(config).expect("Failed to initialize config");
    let cfg = config::Config::get();

    println!("-------------------------------------------------");
    println!("Input Path: {}", cfg.input.display());
    println!("Output Path: {}", cfg.output.display());
    println!("Assets Path: {}", cfg.assets.display());
    println!("WASM Path: {}", cfg.wasm.display());
    println!("Database Path: {}", cfg.database.display());
    println!("Port Number: {}", cfg.port);
    println!("Brand: {}", cfg.brand);
    println!(
        "JWT-token: {}{}",
        &cfg.jwt_token.chars().take(2).collect::<String>(),
        "*".repeat(cfg.jwt_token.len() - 2)
    );
    println!("Flat filename structure: {}", cfg.flat);
    println!("-------------------------------------------------");

    check_pandoc()?;

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
        .route("/assets/*path", get(handlers::serve_internals))
        .route("/wasm/*path", get(handlers::serve_internals))
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
            let l = format!("Listening on: {}", address_config);
            println!("{}", l.green());
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
    let s = "Press Ctrl+C to stop the server...".yellow();
    println!("{s}");
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
