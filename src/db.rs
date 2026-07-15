//! Postgres-Anbindung: Pool aus den DB_*-Umgebungsvariablen bauen und
//! die Migrationen (Schema + Seed) anwenden.

use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};

/// Baut den Connection aus den einzelnen `DB_*`-Variablen (kein fertiger
/// Connection-String in der `.env`) und öffnet einen Pool. Danach werden die
/// Migrationen in `./migrations` angewandt (idempotent über sqlx-Tracking).
pub async fn pool_from_env() -> Result<PgPool, Box<dyn std::error::Error>> {
    let host = std::env::var("DB_HOST")?;
    let port: u16 = std::env::var("DB_PORT")?.parse()?;
    let user = std::env::var("DB_USER")?;
    let pw = std::env::var("DB_PW")?;
    let db = std::env::var("DB_NAME")?;

    // PgConnectOptions statt URL-String: keine URL-Encoding-Fallstricke beim Passwort.
    let opts = PgConnectOptions::new()
        .host(&host)
        .port(port)
        .username(&user)
        .password(&pw)
        .database(&db);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
