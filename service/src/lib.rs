mod api;

mod entities;
pub mod error;
mod migrator;

use axum::{Router, http::StatusCode, routing::post};
use error::Result;
use migrator::Migrator;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tokio::{
    signal,
    sync::mpsc::{self, Receiver},
};

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

async fn shutdown_signal(mut rx: Receiver<()>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
        _ = rx.recv() =>{}
    }
}

pub async fn serve() -> Result<()> {
    init_logger(0);
    let settings = settings::get();
    dbg!(settings);
    let mut opt = ConnectOptions::new(&settings.db.database_url);
    opt.sqlx_logging_level(log::LevelFilter::Debug);

    let conn = Database::connect(opt).await?;

    Migrator::up(&conn, None).await?;

    let state = AppState { conn };

    let (tx, rx) = mpsc::channel::<()>(1);

    let app = spa::spa(
        Router::new()
            .route(
                "/shutdown",
                post(|| async move {
                    let _ = tx.send(()).await;
                    (StatusCode::OK, "shuting down")
                }),
            )
            .nest("/api", api::api_routes())
            .with_state(state),
    );

    let listener =
        tokio::net::TcpListener::bind((settings.server.addr.to_owned(), settings.server.port))
            .await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(rx))
        .await?;
    Ok(())
}

fn init_logger(level: u8) {
    let f = match level {
        0 => "info".into(),
        1 => "debug".into(),
        _ => "trace".into(),
    };
    use tracing_subscriber::{EnvFilter, fmt::time::LocalTime};
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

    let fmt = fmt::layer()
        .with_ansi(true)
        // .with_file(true)
        .with_line_number(true)
        .with_timer(LocalTime::rfc_3339());
    let filter = EnvFilter::try_from_default_env().unwrap_or(f);
    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(fmt)
        .try_init();
}

#[cfg(test)]
mod test {
    use argon2::{
        Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
        password_hash::{SaltString, rand_core::OsRng},
    };
    // use rand::rngs::OsRng;

    #[test]
    fn test_bcrypt_time() {
        let now = std::time::Instant::now();
        let r = bcrypt::verify(
            &"UJU2g1sqL7QcflwEGE",
            &"$2b$10$LKo4l6G9yhXa6/kINbN9zO6JTaMDFs8dKe632w6xZN2WqmMrR.DoK",
        );
        let _ = dbg!(r);
        println!("{}", now.elapsed().as_millis())
    }

    #[test]
    fn test_argon2_hash() -> eyre::Result<()> {
        let password = b"UJU2g1sqL7QcflwEGE"; // Bad password; don't actually use!
        let salt = SaltString::generate(&mut OsRng);

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = argon2.hash_password(password, &salt)?.to_string();
        println!("{}", password_hash);

        let now = std::time::Instant::now();
        // Verify password against PHC string.
        //
        // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
        // `Argon2` instance.
        let parsed_hash = PasswordHash::new(&password_hash)?;
        assert!(
            Argon2::default()
                .verify_password(password, &parsed_hash)
                .is_ok()
        );
        println!("{}", now.elapsed().as_millis());

        Ok(())
    }
}
