//! Gemeinsame Infrastruktur für den On-Prem Azure DevOps Server (mTLS + Basic-Auth).
//!
//! Wird von den Fachmodulen (releasenotes, abkuendigungen, ...) genutzt.
//! Kennt keine Fachlichkeit — nur generische WIQL-Abfrage und Feld-Abruf.
//!
//! Die Abfragen sind hinter dem Trait [`WorkItemSource`] gekapselt, damit die
//! Fachlogik gegen einen Fake testbar ist. `DevOpsClient` ist die echte
//! (netzwerkgebundene) Implementierung.

use async_trait::async_trait;
use reqwest::{Certificate, Client, Identity};
use serde::Deserialize;
use std::collections::HashMap;

/// Send+Sync-Fehler, damit Handler-Futures Send bleiben (Salvo verlangt das).
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

/// Datenquelle für Workitems. Von der Fachlogik genutzt; im Test durch einen
/// Fake ersetzbar. `async_trait` boxt die Futures als `Send` (nötig für Salvo).
#[async_trait]
pub trait WorkItemSource: Send + Sync {
    /// Flache WIQL-Query -> gefundene Workitem-IDs.
    async fn query_ids(&self, wiql: &str) -> Result<Vec<i64>, BoxError>;

    /// WIQL-*WorkItemLinks*-Query -> Ziel-IDs der Verknüpfungen (ohne Wurzel).
    async fn query_linked_ids(&self, wiql: &str) -> Result<Vec<i64>, BoxError>;

    /// Felder für eine Menge von IDs holen.
    async fn work_items(&self, ids: &[i64], fields: &[&str]) -> Result<Vec<WorkItem>, BoxError>;
}

/// Fertig konfigurierter Azure-DevOps-Client. `Clone` ist billig:
/// `reqwest::Client` teilt intern einen `Arc`; nur zwei kurze Strings werden kopiert.
#[derive(Clone)]
pub struct DevOpsClient {
    http: Client,
    base_url: String,
    auth: String, // "Basic <token>"
}

impl DevOpsClient {
    /// Baut den Client EINMAL aus der Env (liest .pfx + Root-CA von der Platte).
    pub fn from_env() -> Result<Self, BoxError> {
        let cfg = Config::from_env()?;

        // .pfx (PKCS#12) inkl. privatem Schlüssel als Client-Identität (mTLS)
        let pfx = std::fs::read(&cfg.cert_path)?;
        let identity = Identity::from_pkcs12_der(&pfx, &cfg.cert_pw)?;

        // Internem Root-CA vertrauen (PEM)
        let ca_pem = std::fs::read(&cfg.root_ca)?;
        let root = Certificate::from_pem(&ca_pem)?;

        let http = Client::builder()
            .add_root_certificate(root)
            .identity(identity)
            .build()?;

        Ok(Self {
            http,
            base_url: cfg.base_url,
            auth: format!("Basic {}", cfg.token),
        })
    }
}

#[async_trait]
impl WorkItemSource for DevOpsClient {
    async fn query_ids(&self, wiql: &str) -> Result<Vec<i64>, BoxError> {
        let body = serde_json::json!({ "query": wiql });
        let resp: WiqlResponse = self
            .http
            .post(format!("{}/_apis/wit/wiql?api-version=7.1", self.base_url))
            .header("Authorization", &self.auth)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(resp.work_items.into_iter().map(|w| w.id).collect())
    }

    async fn query_linked_ids(&self, wiql: &str) -> Result<Vec<i64>, BoxError> {
        let body = serde_json::json!({ "query": wiql });
        let resp: WiqlLinkResponse = self
            .http
            .post(format!("{}/_apis/wit/wiql?api-version=7.1", self.base_url))
            .header("Authorization", &self.auth)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(resp
            .work_item_relations
            .into_iter()
            .filter(|r| r.source.is_some()) // Wurzel (source == null) auslassen
            .filter_map(|r| r.target.map(|t| t.id))
            .collect())
    }

    async fn work_items(&self, ids: &[i64], fields: &[&str]) -> Result<Vec<WorkItem>, BoxError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let ids_csv = ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
        let fields_csv = fields.join(",");
        let resp: WorkItemsResponse = self
            .http
            .get(format!(
                "{}/_apis/wit/workitems?ids={ids_csv}&fields={fields_csv}&api-version=7.1",
                self.base_url
            ))
            .header("Authorization", &self.auth)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(resp.value)
    }
}

/// Ein Workitem mit generischer Feld-Map — jedes Fachmodul zieht sich die
/// Felder heraus, die es braucht (`field_str`).
#[derive(Clone, Deserialize)]
pub struct WorkItem {
    pub id: i64,
    #[serde(default)]
    pub fields: HashMap<String, serde_json::Value>,
}

impl WorkItem {
    /// Feld als String — akzeptiert auch Zahlen/Bool (leer, wenn nicht vorhanden).
    pub fn field_str(&self, name: &str) -> String {
        match self.fields.get(name) {
            Some(serde_json::Value::String(s)) => s.clone(),
            Some(serde_json::Value::Number(n)) => n.to_string(),
            Some(serde_json::Value::Bool(b)) => b.to_string(),
            _ => String::new(),
        }
    }

    /// Feld als i64 (z.B. `System.Parent`); None, wenn nicht vorhanden/keine Zahl.
    pub fn field_i64(&self, name: &str) -> Option<i64> {
        self.fields.get(name).and_then(|v| v.as_i64())
    }
}

// --- interne Formen der Azure-DevOps-Antworten ---

#[derive(Deserialize)]
struct WiqlResponse {
    #[serde(rename = "workItems", default)]
    work_items: Vec<WiqlRef>,
}

#[derive(Deserialize)]
struct WiqlRef {
    id: i64,
}

#[derive(Deserialize)]
struct WiqlLinkResponse {
    #[serde(rename = "workItemRelations", default)]
    work_item_relations: Vec<WiqlLink>,
}

#[derive(Deserialize)]
struct WiqlLink {
    #[serde(default)]
    source: Option<WiqlRef>,
    #[serde(default)]
    target: Option<WiqlRef>,
}

#[derive(Deserialize)]
struct WorkItemsResponse {
    value: Vec<WorkItem>,
}

/// Konfiguration aus Umgebungsvariablen (aus .env via dotenvy geladen).
struct Config {
    base_url: String,  // TFS_AEINS
    token: String,     // TFS_TOKEN (bereits Base64 für "Basic")
    cert_path: String, // TFS_CERT (.pfx)
    cert_pw: String,   // CERT_PW
    root_ca: String,   // ROOT_CA (.cer, PEM)
}

impl Config {
    fn from_env() -> Result<Self, BoxError> {
        let get = |k: &str| -> Result<String, BoxError> {
            std::env::var(k).map_err(|_| format!("Umgebungsvariable {k} fehlt").into())
        };
        Ok(Self {
            base_url: get("TFS_AEINS")?,
            token: get("TFS_TOKEN")?,
            cert_path: get("TFS_CERT")?,
            cert_pw: get("CERT_PW")?,
            root_ca: get("ROOT_CA")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn work_item() -> WorkItem {
        let mut fields = HashMap::new();
        fields.insert("Str".to_string(), serde_json::json!("hallo"));
        fields.insert("Num".to_string(), serde_json::json!(42));
        fields.insert("Bool".to_string(), serde_json::json!(true));
        WorkItem { id: 7, fields }
    }

    #[test]
    fn field_str_handles_string_number_bool_and_missing() {
        let w = work_item();
        assert_eq!(w.field_str("Str"), "hallo");
        assert_eq!(w.field_str("Num"), "42"); // Zahl wird stringifiziert
        assert_eq!(w.field_str("Bool"), "true");
        assert_eq!(w.field_str("fehlt"), ""); // nicht vorhanden -> leer
    }

    #[test]
    fn field_i64_parses_numbers_only() {
        let w = work_item();
        assert_eq!(w.field_i64("Num"), Some(42));
        assert_eq!(w.field_i64("Str"), None);
        assert_eq!(w.field_i64("fehlt"), None);
    }
}
