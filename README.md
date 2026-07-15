# releasenotes

Salvo-basierter HTTP-Service, der aus Azure DevOps (on-prem TFS) die Release-Notes
und Abkündigungen zu A.eins-Versionen erzeugt und veröffentlichte Versionen in
Postgres festhält.

## Voraussetzungen

- **Rust** (installiert) — Projekt nutzt Edition 2024, also aktuelle stabile Toolchain.
- **Docker** — für die Postgres-Datenbank.
- **Zwei Zertifikatsdateien** für den mTLS-Zugriff auf DevOps (nicht im Repo, separat beziehen und ins Projekt-Root legen):
  - Root-CA (`*.cer`, PEM) → in `.env` als `ROOT_CA`
  - Client-Zertifikat (`*.pfx`, PKCS#12, inkl. privatem Schlüssel) → in `.env` als `TFS_CERT`
- Optional: **watchexec** für Auto-Reload — `cargo install watchexec-cli`.

## 1. Konfiguration

```sh
cp .env.example .env
```

Danach in `.env` die echten Werte eintragen:

| Schlüssel | Bedeutung |
|---|---|
| `ROOT_CA`, `TFS_CERT` | Dateinamen der beiden Zertifikate (im Projekt-Root) |
| `CERT_PW` | Passwort des `.pfx` |
| `TFS_TOKEN` | Personal Access Token, Base64-kodiert für Basic-Auth |
| `TFS_AMIC`, `TFS_AEINS` | Basis-URLs des DevOps-Servers |
| `DB_USER`, `DB_PW`, `DB_HOST`, `DB_PORT`, `DB_NAME` | Postgres-Zugang (Defaults passen zum Container unten) |
| `SERVER_HOST` | Bind-Adresse des Servers (Fallback im Code: `0.0.0.0`) |
| `SERVER_PORT` | HTTP-Port des Servers (Fallback im Code: 18080) |

`.env` sowie `*.cer`/`*.pfx` sind per `.gitignore` ausgeschlossen — die Geheimnisse bleiben lokal.

## 2. Postgres starten (Docker)

```sh
docker network create infra
docker volume create releasenotes-data
docker run -d --name releasenotes-db \
  --network infra \
  -v releasenotes-data:/var/lib/postgresql \
  -e POSTGRES_USER=<DB_USER> \
  -e POSTGRES_PASSWORD=<DB_PW> \
  -e POSTGRES_DB=<DB_NAME> \
  -p <DB_PORT>:5432 \
  postgres:18.4-trixie
```

Die Platzhalter `<DB_...>` mit den Werten aus der eigenen `.env` ersetzen.

## 3. Server starten

```sh
cargo run
```

Beim Start wird:

1. die `.env` geladen,
2. der DevOps-Client gebaut (liest die Zertifikate — **fail-fast**),
3. der Postgres-Pool aufgebaut und die Migrationen aus `./migrations` angewandt (Schema + Seed, idempotent),
4. auf `http://localhost:<SERVER_PORT>` gelauscht (Default `18080`).

> **Wichtig:** Die Datenbank muss laufen und die Zertifikate müssen vorhanden sein — sonst bricht der Start ab.

### Auto-Reload während der Entwicklung

```sh
# baut neu und startet den Server, wenn sich etwas in src/ ändert
watchexec -w src -r --shell=none -- cargo run
```

Unter Windows ist `--shell=none` nötig, sonst fügt die Standard-Shell das Kommando zu
einem String zusammen und verschluckt das `run`-Subkommando (cargo zeigt dann nur die Hilfe).

## 4. Tests

```sh
cargo test
```

Die meisten Tests laufen netzwerk- und DB-frei. Der Integrationstest
`persist_release_writes_version_and_abkuendigungen` startet via **Testcontainers**
einen Wegwerf-Postgres und benötigt daher eine **laufende Docker-Engine**.

## Endpoints

| Methode & Pfad | Quelle | Zweck |
|---|---|---|
| `GET /functions` | DevOps | Alle Versionen (Funktionen) als `{id, title, type}` |
| `GET /functions/{id}/workitems` | DevOps | Release-Notes-Workitems einer Funktion (JSON) |
| `GET /functions/{id}/abkuendigungen` | DevOps | Verknüpfte Abkündigungen einer Funktion (JSON) |
| `GET /abkuendigungen` | DevOps | Globale Liste der Abkündigungen |
| `GET /releases` | DB | Alle bekannten (persistierten) Versionsnummern |
| `GET /releases/{versionsnummer}/preview` | DevOps | Vorschau: Release-Notes als `{version}-preview.zip` zum Download, **ohne** zu persistieren |
| `GET /releases/{versionsnummer}/create` | DevOps + DB | Veröffentlichen: persistiert Version + Abkündigungen und liefert die gezippten Release-Notes (409, falls bereits veröffentlicht) |

Die Versionsnummer im `/releases`-Pfad ist der Funktions-Titel, z. B. `9.0.2502.9`.

## Datenbank

Zwei Tabellen, definiert und initial befüllt in `migrations/0001_init.sql` (beim Serverstart angewandt):

- **`versions`** — veröffentlichte Versionen: `workitem_id` (PK), `version`
- **`abkuendigungen`** — `workitem_id` (PK), `titel`, `version_id` → `versions(workitem_id)`

Inhalt inspizieren:

```sh
docker exec -e PGPASSWORD=<DB_PW> releasenotes-db \
  psql -U <DB_USER> -d <DB_NAME> -c "SELECT * FROM versions;"
```

## Nützliches

```sh
# Abhängigkeit mit Features hinzufügen (Features kommasepariert)
cargo add <crate> --features f1,f2,f3
```
