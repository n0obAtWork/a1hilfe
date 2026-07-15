//! Abkündigungen: geschlossene 'Aufgabe'-Workitems der Hilfe-Kategorie 'Abkündigung'.

use crate::devops::{BoxError, DevOpsClient, WorkItemSource};
use salvo::prelude::*;
use serde::Serialize;

// Hinweis: WIQL nutzt '=' (nicht '=='); '<>' bedeutet "ungleich".
const WIQL: &str = "SELECT [System.Id] FROM WorkItems \
                    WHERE [System.WorkItemType] = 'Aufgabe' \
                    AND [AMIC.UseCase.RelnoTitel] <> '' \
                    AND [AMIC.UseCase.HilfeKategorie] = 'Abkündigung' \
                    AND [System.State] = 'Geschlossen'";

/// Routen dieses Moduls (wird in main.rs eingehängt).
pub fn routes() -> Router {
    Router::with_path("abkuendigungen").get(get_all_abkuendigungen)
}

/// GET /abkuendigungen -> [{ id, title, type }, ...]
#[handler]
async fn get_all_abkuendigungen(depot: &mut Depot, res: &mut Response) {
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
            tracing::error!("get_all_abkuendigungen fehlgeschlagen: {e}");
            res.render(
                StatusError::internal_server_error()
                    .brief("Abkündigungen konnten nicht geladen werden."),
            );
        }
    }
}

async fn fetch(client: &DevOpsClient) -> Result<Vec<Abkuendigung>, BoxError> {
    let ids = client.query_ids(WIQL).await?;
    let items = client
        .work_items(&ids, &["System.Id", "System.Title", "System.WorkItemType"])
        .await?;
    Ok(items
        .into_iter()
        .map(|w| Abkuendigung {
            id: w.id,
            title: w.field_str("System.Title"),
            typ: w.field_str("System.WorkItemType"),
        })
        .collect())
}

/// Das, was der Endpoint als JSON ausliefert.
#[derive(Serialize)]
struct Abkuendigung {
    id: i64,
    title: String,
    #[serde(rename = "type")]
    typ: String,
}
