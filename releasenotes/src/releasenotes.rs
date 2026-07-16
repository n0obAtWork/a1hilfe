//! Release-Notes: A.eins-Versionsnummern (Workitems vom Typ 'Funktion').

use crate::devops::{BoxError, DevOpsClient, WorkItemSource};
use salvo::http::header;
use salvo::prelude::*;
use serde::Serialize;
use sqlx::PgPool;
use std::collections::HashMap;

/// Azure-DevOps-Feld mit dem Release-Notes-Freitext (HTML).
const RELNO_TEXT_FIELD: &str = "Microsoft.VSTS.CMMI.ImpactAssessmentHtml";

const WIQL: &str = "SELECT [System.Title] FROM WorkItems \
                    WHERE [System.WorkItemType] = 'Funktion' \
                    AND [System.State] != 'Geschlossen' \
                    AND [System.State] != 'Vorgeschlagen' \
                    AND [System.State] != 'Aktiv' \
                    Order By [System.Title], [Microsoft.VSTS.Scheduling.TargetDate] DESC";

/// Routen dieses Moduls (wird in main.rs eingehängt).
pub fn routes() -> Router {
    Router::with_path("functions")
        .get(get_all_versionnumbers)
        .push(Router::with_path("{id}/workitems").get(get_funktion_workitems))
        .push(Router::with_path("{id}/abkuendigungen").get(get_funktion_abkuendigungen))
}

/// Release-Routen (eigener Pfad `releases/...`, wird in main.rs separat eingehängt):
///   GET /releases                          -> alle bekannten (persistierten) Versionsnummern
///   GET /releases/{versionsnummer}/preview -> ZIP-Download OHNE Persistierung
///   GET /releases/{versionsnummer}/create  -> veröffentlichen: persistieren + ZIP liefern
pub fn release_routes() -> Router {
    Router::with_path("releases")
        .get(get_all_releases)
        .push(
            Router::with_path("{versionsnummer}")
                .push(Router::with_path("preview").get(get_release_preview))
                .push(Router::with_path("create").get(get_release_create)),
        )
}

/// Eine persistierte Version (Zeile aus der `versions`-Tabelle).
#[derive(Serialize)]
struct ReleaseItem {
    #[serde(rename = "workitemId")]
    workitem_id: i64,
    version: String,
}

/// GET /releases
/// Liefert alle bekannten (persistierten) Versionsnummern aus der DB.
#[handler]
async fn get_all_releases(depot: &mut Depot, res: &mut Response) {
    let pool = match depot.get_typed::<PgPool>() {
        Ok(p) => p.clone(),
        Err(_) => {
            res.render(StatusError::internal_server_error().brief("DB nicht verfügbar."));
            return;
        }
    };

    match sqlx::query_as::<_, (i64, String)>(
        "SELECT workitem_id, version FROM versions ORDER BY version",
    )
    .fetch_all(&pool)
    .await
    {
        Ok(rows) => {
            let items: Vec<ReleaseItem> = rows
                .into_iter()
                .map(|(workitem_id, version)| ReleaseItem { workitem_id, version })
                .collect();
            res.render(Json(items));
        }
        Err(e) => {
            tracing::error!("get_all_releases: {e}");
            res.render(
                StatusError::internal_server_error().brief("Releases konnten nicht geladen werden."),
            );
        }
    }
}

/// GET /functions -> [{ id, title, type }, ...]
#[handler]
async fn get_all_versionnumbers(depot: &mut Depot, res: &mut Response) {
    let client = match depot.get_typed::<DevOpsClient>() {
        Ok(c) => c.clone(),
        Err(_) => {
            res.render(StatusError::internal_server_error().brief("Client nicht verfügbar."));
            return;
        }
    };

    match fetch(&client).await {
        Ok(items) => res.render(Json(items)),
        Err(e) => {
            tracing::error!("get_all_versionnumbers fehlgeschlagen: {e}");
            res.render(
                StatusError::internal_server_error()
                    .brief("Versionsnummern konnten nicht geladen werden."),
            );
        }
    }
}

async fn fetch<S: WorkItemSource>(client: &S) -> Result<Vec<VersionItem>, BoxError> {
    let ids = client.query_ids(WIQL).await?;
    let items = client
        .work_items(&ids, &["System.Id", "System.Title", "System.WorkItemType"])
        .await?;
    Ok(items
        .into_iter()
        .map(|w| VersionItem {
            id: w.id,
            title: w.field_str("System.Title"),
            typ: w.field_str("System.WorkItemType"),
        })
        .collect())
}

/// GET /functions/{id}/workitems
/// Liefert die mit der Funktion {id} verknüpften Release-Notes-Workitems
/// (ohne Kategorie 'Maintenance' und 'Abkündigung').
#[handler]
async fn get_funktion_workitems(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let Some(id) = req.param::<i64>("id") else {
        res.render(StatusError::bad_request().brief("Ungültige Funktion-ID."));
        return;
    };
    let client = match depot.get_typed::<DevOpsClient>() {
        Ok(c) => c.clone(),
        Err(_) => {
            res.render(StatusError::internal_server_error().brief("Client nicht verfügbar."));
            return;
        }
    };

    match fetch_linked_workitems(&client, id, CAT_RELEASENOTES).await {
        Ok(items) => res.render(Json(items)),
        Err(e) => {
            tracing::error!("get_funktion_workitems({id}) fehlgeschlagen: {e}");
            res.render(
                StatusError::internal_server_error()
                    .brief("Workitems konnten nicht geladen werden."),
            );
        }
    }
}

/// GET /functions/{id}/abkuendigungen
/// Liefert die mit der Funktion {id} verknüpften Abkündigungen — angereichert
/// (relnoTitel + bereinigter Text), analog zu /workitems, nur Kategorie 'Abkündigung'.
#[handler]
async fn get_funktion_abkuendigungen(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let Some(id) = req.param::<i64>("id") else {
        res.render(StatusError::bad_request().brief("Ungültige Funktion-ID."));
        return;
    };
    let client = match depot.get_typed::<DevOpsClient>() {
        Ok(c) => c.clone(),
        Err(_) => {
            res.render(StatusError::internal_server_error().brief("Client nicht verfügbar."));
            return;
        }
    };

    match fetch_linked_workitems(&client, id, CAT_ABKUENDIGUNG).await {
        Ok(items) => res.render(Json(items)),
        Err(e) => {
            tracing::error!("get_funktion_abkuendigungen({id}) fehlgeschlagen: {e}");
            res.render(
                StatusError::internal_server_error()
                    .brief("Abkündigungen konnten nicht geladen werden."),
            );
        }
    }
}

// Kategorie-Filter für die verknüpften Workitems.
// Release-Notes: alles außer Maintenance UND Abkündigung (letztere kommen separat ans Ende).
const CAT_RELEASENOTES: &str = "AND [Target].[AMIC.UseCase.HilfeKategorie] <> 'Maintenance' \
     AND [Target].[AMIC.UseCase.HilfeKategorie] <> 'Abkündigung'";
// Abkündigungen: nur diese Kategorie.
const CAT_ABKUENDIGUNG: &str = "AND [Target].[AMIC.UseCase.HilfeKategorie] = 'Abkündigung'";

/// Holt die mit der Funktion verknüpften Workitems, gefiltert über `category_where`.
async fn fetch_linked_workitems<S: WorkItemSource>(
    client: &S,
    funk_id: i64,
    category_where: &str,
) -> Result<Vec<WorkItemDto>, BoxError> {
    // funk_id ist als i64 geparst -> nur Ziffern, keine WIQL-Injection möglich.
    let wiql = format!(
        "SELECT [Target].[System.Id] FROM WorkItemLinks \
         WHERE [Source].[System.Id] = {funk_id} \
         {category_where} \
         ORDER BY [Target].[AMIC.UseCase.HilfeKategorie]"
    );
    let ids = client.query_linked_ids(&wiql).await?;
    let items = client
        .work_items(
            &ids,
            &[
                "System.Id",
                "System.Title",
                "System.WorkItemType",
                "System.Parent",
                "AMIC.UseCase.RelnoTitel",
                "AMIC.UseCase.HilfeKategorie",
                "AMIC.UseCase.RelnoAnw",
                "AMIC.UseCase.RelnoVariante",
                "AMIC.UseCase.RelnoFuncOrReport",
                RELNO_TEXT_FIELD,
            ],
        )
        .await?;

    // Ticket-Fallnummer stammt aus dem übergeordneten Workitem (Typ 'Anforderung').
    // Parent-IDs sammeln und deren WlProjektnr in einem Batch auflösen.
    let parent_ids: Vec<i64> = items.iter().filter_map(|w| w.field_i64("System.Parent")).collect();
    let wl_by_parent: HashMap<i64, String> = client
        .work_items(&parent_ids, &["System.Id", "AMIC.UseCase.CS.WlProjektnr"])
        .await?
        .into_iter()
        .map(|p| (p.id, p.field_str("AMIC.UseCase.CS.WlProjektnr")))
        .collect();

    Ok(items
        .into_iter()
        .map(|w| {
            // Ticket = <WlProjektnr des Parents>[<workitem.id>]
            let wl = w
                .field_i64("System.Parent")
                .and_then(|pid| wl_by_parent.get(&pid))
                .cloned()
                .unwrap_or_default();
            WorkItemDto {
                ticket: format!("{wl}[{}]", w.id),
                anwendung: w.field_str("AMIC.UseCase.RelnoAnw"),
                variante: w.field_str("AMIC.UseCase.RelnoVariante"),
                funktion_report: w.field_str("AMIC.UseCase.RelnoFuncOrReport"),
                relno_titel: w.field_str("AMIC.UseCase.RelnoTitel"),
                hilfe_kategorie: w.field_str("AMIC.UseCase.HilfeKategorie"),
                relno_text: html_to_text(&w.field_str(RELNO_TEXT_FIELD)),
                title: w.field_str("System.Title"),
                typ: w.field_str("System.WorkItemType"),
                id: w.id,
            }
        })
        .collect())
}

/// Ein zur Funktion gehörendes Workitem.
#[derive(Serialize)]
struct WorkItemDto {
    id: i64,
    title: String,
    #[serde(rename = "type")]
    typ: String,
    #[serde(rename = "relnoTitel")]
    relno_titel: String,
    #[serde(rename = "hilfeKategorie")]
    hilfe_kategorie: String,
    #[serde(rename = "relnoText")]
    relno_text: String,
    ticket: String,
    anwendung: String,
    variante: String,
    #[serde(rename = "funktionReport")]
    funktion_report: String,
}

/// Das, was der Endpoint als JSON ausliefert.
#[derive(Serialize)]
struct VersionItem {
    id: i64,
    title: String,
    #[serde(rename = "type")]
    typ: String,
}

/// Statusmeldung, z.B. wenn eine Version bereits veröffentlicht ist.
#[derive(Serialize)]
struct ReleaseStatus {
    version: String,
    published: bool,
    message: String,
}

/// Einfache Meldung (z.B. „keine bekannte Versionsnummer").
#[derive(Serialize)]
struct ReleaseMessage {
    version: String,
    message: String,
}

/// Ist die Versionsnummer bereits in der DB (= veröffentlicht)? Reiner DB-Check.
async fn version_published(pool: &PgPool, version: &str) -> Result<bool, sqlx::Error> {
    let row = sqlx::query_scalar::<_, i64>("SELECT workitem_id FROM versions WHERE version = $1")
        .bind(version)
        .fetch_optional(pool)
        .await?;
    Ok(row.is_some())
}

/// Persistiert eine Version + ihre Abkündigungen in einer Transaktion.
async fn persist_release(
    pool: &PgPool,
    funk_id: i64,
    version: &str,
    abks: &[WorkItemDto],
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("INSERT INTO versions (workitem_id, version) VALUES ($1, $2) ON CONFLICT (workitem_id) DO NOTHING")
        .bind(funk_id)
        .bind(version)
        .execute(&mut *tx)
        .await?;
    for a in abks {
        sqlx::query(
            "INSERT INTO abkuendigungen (workitem_id, titel, version_id) VALUES ($1, $2, $3) \
             ON CONFLICT (workitem_id) DO UPDATE \
             SET titel = EXCLUDED.titel, version_id = EXCLUDED.version_id",
        )
        .bind(a.id)
        .bind(&a.relno_titel)
        .bind(funk_id)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(())
}

/// Schreibt die ZIP-Bytes als Datei-Download in die Response.
fn write_zip(res: &mut Response, basename: &str, bytes: Vec<u8>) {
    let _ = res.add_header(header::CONTENT_TYPE, "application/zip", true);
    let _ = res.add_header(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{basename}.zip\""),
        true,
    );
    res.body(bytes);
}

/// GET /releases/{versionsnummer}/preview
/// Erzeugt die gezippten Release-Notes zum Download — OHNE zu persistieren.
/// Zum Ansehen/Prüfen, bevor per /create veröffentlicht wird.
#[handler]
async fn get_release_preview(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let Some(versionsnummer) = req.param::<String>("versionsnummer") else {
        res.render(StatusError::bad_request().brief("Ungültige Versionsnummer."));
        return;
    };
    let client = match depot.get_typed::<DevOpsClient>() {
        Ok(c) => c.clone(),
        Err(_) => {
            res.render(StatusError::internal_server_error().brief("Client nicht verfügbar."));
            return;
        }
    };

    // Versionsnummer (Titel) über die globale Funktionsliste zur Funktions-ID auflösen.
    let funk_id = match fetch(&client).await {
        Ok(items) => items.into_iter().find(|i| i.title == versionsnummer).map(|i| i.id),
        Err(e) => {
            tracing::error!("get_release_preview fetch('{versionsnummer}'): {e}");
            res.render(StatusError::internal_server_error().brief("Versionen konnten nicht geladen werden."));
            return;
        }
    };
    let Some(funk_id) = funk_id else {
        res.status_code(StatusCode::NOT_FOUND);
        res.render(Json(ReleaseMessage {
            version: versionsnummer.clone(),
            message: format!("'{versionsnummer}' ist keine bekannte Versionsnummer."),
        }));
        return;
    };

    // ZIP (mit "-preview"-Namenszusatz) erzeugen und ausliefern — ohne zu persistieren.
    match build_zip(&client, funk_id, "-preview").await {
        Ok((basename, bytes)) => write_zip(res, &basename, bytes),
        Err(e) => {
            tracing::error!("get_release_preview build('{versionsnummer}'): {e}");
            res.render(StatusError::internal_server_error().brief("ZIP konnte nicht erzeugt werden."));
        }
    }
}

/// GET /releases/{versionsnummer}/create
/// Veröffentlicht die Version: prüft, ob sie bereits existiert (dann Konflikt-
/// Meldung), sonst persistiert sie Funktion + Abkündigungen in der DB und liefert
/// die gezippte Markdown-Datei. Der Pfad-Parameter ist die Versionsnummer
/// (= Titel der Funktion); Name von ZIP und MD ist ebenfalls die Versionsnummer.
#[handler]
async fn get_release_create(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let Some(versionsnummer) = req.param::<String>("versionsnummer") else {
        res.render(StatusError::bad_request().brief("Ungültige Versionsnummer."));
        return;
    };
    let client = match depot.get_typed::<DevOpsClient>() {
        Ok(c) => c.clone(),
        Err(_) => {
            res.render(StatusError::internal_server_error().brief("Client nicht verfügbar."));
            return;
        }
    };
    let pool = match depot.get_typed::<PgPool>() {
        Ok(p) => p.clone(),
        Err(_) => {
            res.render(StatusError::internal_server_error().brief("DB nicht verfügbar."));
            return;
        }
    };

    // Bereits veröffentlicht? -> JSON-Meldung mit Status 409 (Conflict), nichts persistieren.
    match version_published(&pool, &versionsnummer).await {
        Ok(true) => {
            res.status_code(StatusCode::CONFLICT);
            res.render(Json(ReleaseStatus {
                version: versionsnummer.clone(),
                published: true,
                message: format!(
                    "Für Version '{versionsnummer}' wurde bereits ein Releasenote erstellt."
                ),
            }));
            return;
        }
        Ok(false) => {}
        Err(e) => {
            tracing::error!("get_release_create db-check('{versionsnummer}'): {e}");
            res.render(StatusError::internal_server_error().brief("DB-Prüfung fehlgeschlagen."));
            return;
        }
    }

    // Versionsnummer (Titel) über die globale Funktionsliste zur Funktions-ID auflösen.
    let funk_id = match fetch(&client).await {
        Ok(items) => items.into_iter().find(|i| i.title == versionsnummer).map(|i| i.id),
        Err(e) => {
            tracing::error!("get_release_create fetch('{versionsnummer}'): {e}");
            res.render(StatusError::internal_server_error().brief("Versionen konnten nicht geladen werden."));
            return;
        }
    };
    let Some(funk_id) = funk_id else {
        res.status_code(StatusCode::NOT_FOUND);
        res.render(Json(ReleaseMessage {
            version: versionsnummer.clone(),
            message: format!("'{versionsnummer}' ist keine bekannte Versionsnummer."),
        }));
        return;
    };

    // Abkündigungen der Funktion holen (werden mitpersistiert).
    let abks = match fetch_linked_workitems(&client, funk_id, CAT_ABKUENDIGUNG).await {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("get_release_create abk('{versionsnummer}'): {e}");
            res.render(StatusError::internal_server_error().brief("Abkündigungen konnten nicht geladen werden."));
            return;
        }
    };

    // Persistieren (Version + Abkündigungen) in einer Transaktion.
    if let Err(e) = persist_release(&pool, funk_id, &versionsnummer, &abks).await {
        tracing::error!("get_release_create persist('{versionsnummer}'): {e}");
        res.render(StatusError::internal_server_error().brief("Persistieren fehlgeschlagen."));
        return;
    }

    match build_zip(&client, funk_id, "").await {
        Ok((basename, bytes)) => write_zip(res, &basename, bytes),
        Err(e) => {
            tracing::error!("get_release_create build('{versionsnummer}'): {e}");
            res.render(StatusError::internal_server_error().brief("ZIP konnte nicht erzeugt werden."));
        }
    }
}

/// Baut die gezippte MD im Speicher. `suffix` wird an den Dateinamen (ZIP + MD)
/// angehängt (z.B. "-preview"); die Versionsangabe IM Markdown bleibt unverändert.
/// Rückgabe: (Basis-Dateiname, ZIP-Bytes).
async fn build_zip<S: WorkItemSource>(client: &S, funk_id: i64, suffix: &str) -> Result<(String, Vec<u8>), BoxError> {
    // Versionsnummer = Titel der Funktion selbst
    let funk = client.work_items(&[funk_id], &["System.Title"]).await?;
    let version = funk
        .first()
        .map(|w| w.field_str("System.Title"))
        .unwrap_or_default();
    if version.is_empty() {
        return Err(format!("Funktion {funk_id} nicht gefunden").into());
    }

    // Release-Notes-Workitems (nach Hilfekategorie, dann relnoTitel sortiert)
    let sort = |list: &mut Vec<WorkItemDto>| {
        list.sort_by(|a, b| {
            a.hilfe_kategorie
                .cmp(&b.hilfe_kategorie)
                .then_with(|| a.relno_titel.cmp(&b.relno_titel))
        });
    };
    let mut releasenotes = fetch_linked_workitems(client, funk_id, CAT_RELEASENOTES).await?;
    sort(&mut releasenotes);
    // Abkündigungen (kommen ans Ende), nach relnoTitel sortiert
    let mut abkuendigungen = fetch_linked_workitems(client, funk_id, CAT_ABKUENDIGUNG).await?;
    abkuendigungen.sort_by(|a, b| a.relno_titel.cmp(&b.relno_titel));

    let md = build_markdown(&version, &releasenotes, &abkuendigungen);
    let basename = format!("{version}{suffix}");
    let bytes = zip_single_file(&format!("{basename}.md"), md.as_bytes())?;
    Ok((basename, bytes))
}

/// Erzeugt das Markdown: H1 = Releasenotes, H2 = Hilfekategorie, H3 = relnoTitel,
/// darunter Text + "Releasenote Kategorie"-Block. Abkündigungen hängen unter
/// einem eigenen H2 'Abkündigungen' am Ende.
fn build_markdown(version: &str, releasenotes: &[WorkItemDto], abkuendigungen: &[WorkItemDto]) -> String {
    let mut md = String::from("# Releasenotes\n\n");

    // Release-Notes nach Hilfekategorie gruppiert
    let mut current: Option<&str> = None;
    for w in releasenotes {
        let cat = if w.hilfe_kategorie.is_empty() {
            "Ohne Kategorie"
        } else {
            w.hilfe_kategorie.as_str()
        };
        if current != Some(cat) {
            md.push_str(&format!("## {cat}\n\n"));
            current = Some(cat);
        }
        push_item(&mut md, version, w);
    }

    // Abkündigungen am Ende (nur wenn vorhanden)
    if !abkuendigungen.is_empty() {
        md.push_str("## Abkündigungen\n\n");
        for w in abkuendigungen {
            push_item(&mut md, version, w);
        }
    }

    md
}

/// Rendert einen Workitem-Eintrag: H3 = relnoTitel, Text, "Releasenote Kategorie"-Block.
fn push_item(md: &mut String, version: &str, w: &WorkItemDto) {
    md.push_str(&format!("### {}\n\n", w.relno_titel));
    if !w.relno_text.is_empty() {
        md.push_str(&w.relno_text);
        md.push_str("\n\n");
    }
    // Zwei Leerzeichen am Zeilenende = harter Markdown-Umbruch (Felder als eigene Zeilen).
    md.push_str("**Releasenote Kategorie:**\n\n");
    md.push_str(&format!("Ticket: {}  \n", w.ticket));
    md.push_str(&format!("Version: {version}  \n"));
    md.push_str(&format!("Anwendung: {}  \n", dash(&w.anwendung)));
    md.push_str(&format!("Variante: {}  \n", dash(&w.variante)));
    md.push_str(&format!("Funktion/Report: {}  \n", dash(&w.funktion_report)));
    md.push('\n');
}

/// Leeren Wert als "--" darstellen (wie im Original-Layout).
fn dash(s: &str) -> &str {
    if s.trim().is_empty() { "--" } else { s }
}

/// Zippt einen einzelnen Datei-Eintrag im Speicher (keine Zwischendatei auf Platte).
fn zip_single_file(name: &str, content: &[u8]) -> Result<Vec<u8>, BoxError> {
    use std::io::Write;
    use zip::write::SimpleFileOptions;

    let mut cursor = std::io::Cursor::new(Vec::new());
    {
        let mut zip = zip::ZipWriter::new(&mut cursor);
        let opts = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        zip.start_file(name, opts)?;
        zip.write_all(content)?;
        zip.finish()?;
    }
    Ok(cursor.into_inner())
}

/// Einfache HTML->Text-Bereinigung: entfernt Tags, macht aus Block-/Zeilen-
/// Elementen Zeilenumbrüche und dekodiert die häufigsten Entities.
fn html_to_text(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut chars = html.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '<' {
            // Tag bis '>' einlesen
            let mut tag = String::new();
            for nc in chars.by_ref() {
                if nc == '>' {
                    break;
                }
                tag.push(nc);
            }
            // Tag-Name (ohne führendes '/') -> Block-Elemente werden zu Umbrüchen
            let name: String = tag
                .trim()
                .trim_start_matches('/')
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric())
                .collect();
            match name.to_ascii_lowercase().as_str() {
                "br" | "p" | "div" | "li" | "tr" | "ul" | "ol" => out.push('\n'),
                _ => {}
            }
        } else {
            out.push(c);
        }
    }

    // Entities dekodieren (amp zuletzt, damit &amp;lt; korrekt wird)
    let out = out
        .replace("&nbsp;", " ")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&");

    // Zeilen trimmen, Mehrfach-Leerzeilen auf eine reduzieren
    let mut result = String::new();
    let mut blank = false;
    for line in out.lines() {
        let l = line.trim();
        if l.is_empty() {
            if !blank {
                result.push('\n');
            }
            blank = true;
        } else {
            result.push_str(l);
            result.push('\n');
            blank = false;
        }
    }
    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devops::WorkItem;
    use async_trait::async_trait;
    use salvo::test::TestClient;
    use std::collections::HashMap;

    /// WorkItem aus einem JSON-Objekt bauen (Feldwerte dürfen Strings/Zahlen sein).
    fn wi(id: i64, fields: serde_json::Value) -> WorkItem {
        WorkItem {
            id,
            fields: serde_json::from_value(fields).unwrap(),
        }
    }

    /// Fake-Datenquelle: `work_items` liefert die hinterlegten Items zu den IDs,
    /// `query_*` liefern eine feste ID-Liste. Kein Netzwerk.
    struct FakeSource {
        ids: Vec<i64>,
        items: HashMap<i64, WorkItem>,
    }

    #[async_trait]
    impl WorkItemSource for FakeSource {
        async fn query_ids(&self, _wiql: &str) -> Result<Vec<i64>, BoxError> {
            Ok(self.ids.clone())
        }
        async fn query_linked_ids(&self, _wiql: &str) -> Result<Vec<i64>, BoxError> {
            Ok(self.ids.clone())
        }
        async fn work_items(&self, ids: &[i64], _fields: &[&str]) -> Result<Vec<WorkItem>, BoxError> {
            Ok(ids.iter().filter_map(|id| self.items.get(id).cloned()).collect())
        }
    }

    #[tokio::test]
    async fn fetch_linked_resolves_ticket_and_maps_fields() {
        let items = HashMap::from([
            (
                38591,
                wi(
                    38591,
                    serde_json::json!({
                        "System.Title": "Ergebnisanzeige OSQLFenster",
                        "System.WorkItemType": "Aufgabe",
                        "System.Parent": 38559,
                        "AMIC.UseCase.RelnoTitel": "Statusmeldung in OSQL",
                        "AMIC.UseCase.HilfeKategorie": "Allgemeine Programmfunktionen",
                        "AMIC.UseCase.RelnoAnw": "OSQL",
                        "Microsoft.VSTS.CMMI.ImpactAssessmentHtml": "<p>Wieder da.</p>"
                    }),
                ),
            ),
            // Parent-Anforderung liefert die WlProjektnr (als Zahl)
            (
                38559,
                wi(
                    38559,
                    serde_json::json!({
                        "System.WorkItemType": "Anforderung",
                        "AMIC.UseCase.CS.WlProjektnr": 751529
                    }),
                ),
            ),
        ]);
        let src = FakeSource { ids: vec![38591], items };

        let result = fetch_linked_workitems(&src, 999, CAT_RELEASENOTES).await.unwrap();
        assert_eq!(result.len(), 1);
        let w = &result[0];
        assert_eq!(w.ticket, "751529[38591]"); // WlProjektnr(Parent) + [id]
        assert_eq!(w.relno_titel, "Statusmeldung in OSQL");
        assert_eq!(w.anwendung, "OSQL");
        assert_eq!(w.hilfe_kategorie, "Allgemeine Programmfunktionen");
        assert_eq!(w.relno_text, "Wieder da."); // HTML entfernt
    }

    #[tokio::test]
    async fn fetch_linked_ticket_without_parent_is_bracketed_id() {
        let items = HashMap::from([(
            100,
            wi(
                100,
                serde_json::json!({
                    "System.Title": "Ohne Parent",
                    "AMIC.UseCase.RelnoTitel": "Titel",
                    "AMIC.UseCase.HilfeKategorie": "AIS"
                }),
            ),
        )]);
        let src = FakeSource { ids: vec![100], items };
        let result = fetch_linked_workitems(&src, 1, CAT_RELEASENOTES).await.unwrap();
        assert_eq!(result[0].ticket, "[100]"); // kein Parent -> leer vor der Klammer
    }

    fn dto(cat: &str, titel: &str, text: &str, ticket: &str, anwendung: &str, variante: &str) -> WorkItemDto {
        WorkItemDto {
            id: 1,
            title: "intern".into(),
            typ: "Aufgabe".into(),
            relno_titel: titel.into(),
            hilfe_kategorie: cat.into(),
            relno_text: text.into(),
            ticket: ticket.into(),
            anwendung: anwendung.into(),
            variante: variante.into(),
            funktion_report: String::new(),
        }
    }

    #[test]
    fn html_to_text_strips_tags_and_decodes_entities() {
        assert_eq!(html_to_text("<b>fett</b>"), "fett");
        assert_eq!(html_to_text("a<br>b"), "a\nb");
        assert_eq!(html_to_text("x &lt;y&gt; &amp; z"), "x <y> & z");
        assert!(!html_to_text("<div>text</div>").contains('<'));
    }

    #[test]
    fn dash_replaces_empty() {
        assert_eq!(dash(""), "--");
        assert_eq!(dash("   "), "--");
        assert_eq!(dash("OSQL"), "OSQL");
    }

    #[test]
    fn markdown_has_title_categories_and_kategorie_block() {
        let rn = vec![
            dto("AIS", "Titel A", "Text A", "751529[38591]", "OSQL", ""),
            dto("AIS", "Titel B", "Text B", "42[2]", "", "V1"),
        ];
        let md = build_markdown("9.0.2401.1", &rn, &[]);

        assert!(md.starts_with("# Releasenotes"));
        assert_eq!(md.matches("## AIS").count(), 1); // gleiche Kategorie -> EINE Überschrift
        assert!(md.contains("### Titel A"));
        assert!(md.contains("### Titel B"));
        assert!(md.contains("**Releasenote Kategorie:**"));
        assert!(md.contains("Ticket: 751529[38591]"));
        assert!(md.contains("Version: 9.0.2401.1"));
        assert!(md.contains("Anwendung: OSQL"));
        assert!(md.contains("Anwendung: --")); // leeres Feld -> --
        assert!(!md.contains("## Abkündigungen")); // keine -> kein Abschnitt
    }

    #[test]
    fn markdown_appends_abkuendigungen_at_end() {
        let rn = vec![dto("AIS", "RN", "text", "1[1]", "App", "")];
        let ab = vec![dto("Abkündigung", "Abk 1", "abk text", "2[2]", "App2", "")];
        let md = build_markdown("9.0", &rn, &ab);

        assert!(md.contains("## Abkündigungen"));
        assert!(md.contains("### Abk 1"));
        // Abkündigungen kommen NACH den Release-Notes-Kategorien
        assert!(md.find("## AIS").unwrap() < md.find("## Abkündigungen").unwrap());
    }

    #[tokio::test]
    async fn invalid_function_id_returns_400() {
        let service = Service::new(routes());
        let res = TestClient::get("http://x/functions/abc/workitems")
            .send(&service)
            .await;
        assert_eq!(res.status_code, Some(StatusCode::BAD_REQUEST));
    }

    // --- Release-Routen: netzwerkfreie Routing-Tests ---
    // Ohne DevOpsClient/PgPool im Depot bricht der Handler VOR jedem Netzwerk-/DB-
    // Zugriff mit 500 ab. Ein 500 (statt 404) beweist: die Route ist verdrahtet.

    #[tokio::test]
    async fn releases_list_route_is_wired() {
        let service = Service::new(release_routes());
        let res = TestClient::get("http://x/releases").send(&service).await;
        assert_eq!(res.status_code, Some(StatusCode::INTERNAL_SERVER_ERROR));
    }

    #[tokio::test]
    async fn release_preview_route_is_wired() {
        let service = Service::new(release_routes());
        let res = TestClient::get("http://x/releases/9.0.2502.9/preview").send(&service).await;
        assert_eq!(res.status_code, Some(StatusCode::INTERNAL_SERVER_ERROR));
    }

    #[tokio::test]
    async fn release_create_route_is_wired() {
        let service = Service::new(release_routes());
        let res = TestClient::get("http://x/releases/9.0.2502.9/create").send(&service).await;
        assert_eq!(res.status_code, Some(StatusCode::INTERNAL_SERVER_ERROR));
    }

    // --- Persistenz-Integrationstest (benötigt eine laufende Docker-Engine) ---
    // Startet via Testcontainers einen Wegwerf-Postgres, wendet die Migrationen an
    // und prüft persist_release + version_published gegen eine echte DB.
    #[tokio::test]
    async fn persist_release_writes_version_and_abkuendigungen() {
        use testcontainers::ImageExt;
        use testcontainers::runners::AsyncRunner;
        use testcontainers_modules::postgres::Postgres;

        // Postgres-Image-Tag konfigurierbar (Env TEST_POSTGRES_TAG), Default = Dev/Prod-Version.
        // (Crate-Default wäre sonst postgres:11-alpine.)
        let pg_tag = std::env::var("TEST_POSTGRES_TAG").unwrap_or_else(|_| "18.4-trixie".to_string());
        let node = Postgres::default()
            .with_tag(pg_tag)
            .start()
            .await
            .expect("Container-Start");
        let host = node.get_host().await.expect("host");
        let port = node.get_host_port_ipv4(5432).await.expect("port");
        let url = format!("postgres://postgres:postgres@{host}:{port}/postgres");

        let pool = sqlx::PgPool::connect(&url).await.expect("connect");
        sqlx::migrate!("./migrations").run(&pool).await.expect("migrate");

        // Neue, nicht geseedete Version veröffentlichen.
        assert!(!version_published(&pool, "9.9.9999.9").await.unwrap());

        let abks = vec![dto("Abkündigung", "Test-Abk", "text", "1[1]", "App", "")]; // dto() -> id = 1
        persist_release(&pool, 99999, "9.9.9999.9", &abks).await.expect("persist");

        // Version persistiert?
        let version: String =
            sqlx::query_scalar("SELECT version FROM versions WHERE workitem_id = 99999")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(version, "9.9.9999.9");

        // Abkündigung mit version_id verknüpft?
        let vid: i64 =
            sqlx::query_scalar("SELECT version_id FROM abkuendigungen WHERE workitem_id = 1")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(vid, 99999);

        // Jetzt gilt die Version als veröffentlicht.
        assert!(version_published(&pool, "9.9.9999.9").await.unwrap());
    }
}
