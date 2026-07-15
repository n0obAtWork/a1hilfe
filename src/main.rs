use salvo::affix_state;
use salvo::prelude::*;

mod abkuendigungen;
mod db;
mod devops;
mod releasenotes;

use devops::DevOpsClient;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // .env laden, damit TFS_*/CERT_*/ROOT_CA als Umgebungsvariablen verfügbar sind
    dotenvy::dotenv().ok();

    // DevOps-Client EINMAL bauen (fail-fast) und global injizieren,
    // damit alle Fachmodule (releasenotes, abkuendigungen) ihn teilen.
    let devops = DevOpsClient::from_env().expect("DevOps-Client konnte nicht initialisiert werden");

    // Postgres-Pool bauen und Migrationen (Schema + Seed) anwenden.
    let pool = db::pool_from_env()
        .await
        .expect("DB-Pool konnte nicht initialisiert werden");

    let router = Router::new()
        .hoop(affix_state::inject(devops))
        .hoop(affix_state::inject(pool))
        .push(releasenotes::routes())
        .push(releasenotes::release_routes())
        .push(abkuendigungen::routes());

    // Bind-Adresse aus der Umgebung (SERVER_HOST/SERVER_PORT), sonst Fallbacks.
    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("SERVER_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);

    // Reines HTTP (kein TLS): der Server lauscht auf der konfigurierten Adresse.
    let acceptor = TcpListener::new(format!("{host}:{port}")).bind().await;

    Server::new(acceptor).serve(router).await;
}
