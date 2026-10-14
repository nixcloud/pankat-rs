
mod articles;
mod auth;
mod config;
mod db;
mod error;
mod file_monitor;
mod fingerprint;
mod handlers;
mod registry;
mod renderer;
#[cfg(test)]
mod test_config;
#[cfg(test)]
mod router_tests;
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

fn create_router(pool: db::DbPool, subdir: &str) -> Router {
    let app = Router::new()
        .nest(
            subdir,
            Router::new()
                .route("/posts/*path", get(handlers::serve_input))
                .route("/github-webhook", post(handlers::github_webhook))
                .route("/media/*path", get(handlers::serve_input))
                .route("/assets/*path", get(handlers::serve_internals))
                .route("/wasm/*path", get(handlers::serve_internals))
                .route("/login", get(handlers::login_form).post(handlers::login))
                .route("/logout", post(handlers::logout))
                .route("/api/session", get(handlers::session))
                .route("/draft", get(handlers::drafts_redirect))
                .route("/drafts", get(handlers::drafts_redirect))
                .route("/drafts/", get(handlers::drafts_index))
                .route("/drafts/*path", get(handlers::draft))
                .route("/specialpages", get(handlers::special_pages_redirect))
                .route("/specialpages/", get(handlers::special_pages_index))
                .route("/specialpages/index.html", get(handlers::special_pages_index))
                .route("/api/ws", get(handlers::websocket_route))
                .route("/", get(handlers::serve_index))
                .route("/*path", get(handlers::serve_output))
                .layer(CorsLayer::permissive())
                .with_state(pool),
        )
        // requests outside of subdir, inside it /*path renders the 404 page
        .fallback(handlers::not_found);
    // nest() doesn't pass "{subdir}/" to the nested router, only "{subdir}" and "{subdir}/..."
    match subdir {
        "" => app,
        _ => app.route(&format!("{}/", subdir), get(handlers::serve_index)),
    }
}

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
            Arg::new("site_url")
                .short('b')
                .long("site_url")
                .value_name("URL")
                .help("Public base URL of the site incl. subdir, e.g. https://lastlog.de/blog (used for the label top left and Open Graph tags)")
                .required(false)
                .default_value("https://lastlog.de/blog")
        )
        .arg(
            Arg::new("subdir")
                .short('u')
                .long("subdir")
                .help("A subdirectory the blog is in, can be /blog or even empty")
                .required(false)
                .default_value("/blog")
        )
        .arg(
            Arg::new("jwt_token")
                .long("jwt_token")
                .value_name("STRING")
                .help("Server side secret which signs the admin session cookies, changing it logs out all sessions")
                .required(false)
        )
        .arg(
            Arg::new("admin_password")
                .long("admin_password")
                .value_name("STRING")
                .help("Password of the admin, the only user, for the /login page")
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
            Arg::new("rebuild")
                .long("rebuild")
                .help("Ignore what is known about earlier builds: parse all articles, run pandoc and write all html again")
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
        "subdir".to_string(),
        ConfigValue {
            value: ConfigValueType::Path(matches.get_one::<String>("subdir").map(|v| {
                // println!("{v}");
                // println!("{:?}", std::path::Path::new(v));
                std::path::Path::new(v).into()
            })),
            is_default: Some(clap::parser::ValueSource::DefaultValue)
                == matches.value_source("subdir"),
        },
    );

    config_values.insert(
        "site_url".to_string(),
        ConfigValue {
            value: ConfigValueType::String(matches.get_one::<String>("site_url").map(|v| v.into())),
            is_default: Some(clap::parser::ValueSource::DefaultValue)
                == matches.value_source("site_url"),
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
        "admin_password".to_string(),
        ConfigValue {
            value: ConfigValueType::String(
                matches
                    .get_one::<String>("admin_password")
                    .map(|v| v.into()),
            ),
            is_default: Some(clap::parser::ValueSource::DefaultValue)
                == matches.value_source("admin_password"),
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
    println!("Subdir: {}", cfg.subdir.display());
    println!("Site URL: {}", cfg.site_url);
    println!("Port Number: {}", cfg.port);
    println!(
        "JWT-token: {}{}",
        &cfg.jwt_token.chars().take(2).collect::<String>(),
        "*".repeat(cfg.jwt_token.len().saturating_sub(2))
    );
    println!("Admin password: {}", "*".repeat(cfg.admin_password.len()));
    println!("Flat filename structure: {}", cfg.flat);
    println!("-------------------------------------------------");

    check_pandoc()?;
    if let Err(e) = renderer::assets::Asset::check_all() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    // Initialize SQLite database with Diesel
    let pool = db::establish_connection_pool();
    {
        let mut conn = pool.get().expect("Failed to get a connection from the pool");
        if matches.get_flag("rebuild") {
            println!("--rebuild: forgetting all fingerprints");
            db::build_state::forget_fingerprints(&mut conn)?;
        }
        fingerprint::report_changes(&mut conn);
    }

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

    let binding = cfg.subdir.clone().display().to_string();
    let subdir = binding.as_str();

    let app = create_router(pool.clone(), subdir);

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
