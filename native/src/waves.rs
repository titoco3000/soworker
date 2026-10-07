//! Waves (docs/chat-and-waves.md). Sending: a gaia `emote` plus `POST waveToUser/create`. Receiving: gaia shows the
//! emote at once, and SoWork keeps a notification `users_sys/<me>/notifications/<id>` (type `wave-to-user`) until the
//! recipient dismisses it with `POST notifications/markAs`.
use crate::proto::Net;
use crate::session::CLIENT_VERSION;
use serde_json::{json, Value};
use std::time::Duration;

const FIRESTORE: &str = "https://firestore.googleapis.com/v1/projects/sage-learning-prod/databases/(default)/documents";

#[derive(Clone, Debug, PartialEq)]
pub struct WaveNote {
    /// The notification document's id.
    pub id: String,
    pub from: String,
    pub name: String,
}

/// `runQuery` answer -> the unread wave notifications in it.
pub fn parse_notes(resp: &Value) -> Vec<WaveNote> {
    let s = |v: &Value, path: &str| v.pointer(path).and_then(Value::as_str).map(String::from);
    resp.as_array()
        .into_iter()
        .flatten()
        .filter_map(|row| {
            let d = row.get("document")?;
            let f = |p: &str| s(d, &format!("/fields{p}/stringValue"));
            if f("/type")? != "wave-to-user" {
                return None;
            }
            Some(WaveNote {
                id: s(d, "/name")?.rsplit('/').next()?.to_string(),
                from: f("/data/mapValue/fields/from/mapValue/fields/userId")?,
                name: f("/data/mapValue/fields/from/mapValue/fields/userName").unwrap_or_default(),
            })
        })
        .collect()
}

/// Notifications not yet removed, filtered to waves.
pub fn pending(token: &str, uid: &str) -> Result<Vec<WaveNote>, String> {
    let body = json!({"structuredQuery": {
        "from": [{"collectionId": "notifications"}],
        "where": {"fieldFilter": {"field": {"fieldPath": "removed"}, "op": "EQUAL", "value": {"booleanValue": false}}},
        "limit": 50}});
    let resp: Value = ureq::post(&format!("{FIRESTORE}/users_sys/{uid}:runQuery"))
        .set("authorization", &format!("Bearer {token}"))
        .timeout(Duration::from_secs(10))
        .send_json(body)
        .map_err(|e| format!("wave notifications: {e}"))?
        .into_json()
        .map_err(|e| e.to_string())?;
    Ok(parse_notes(&resp))
}

pub fn pending_async(token: String, uid: String, tx: std::sync::mpsc::Sender<Net>) {
    std::thread::spawn(move || match pending(&token, &uid) {
        Ok(n) => drop(tx.send(Net::Waves(n))),
        Err(e) => eprintln!("{e}"),
    });
}

fn post(token: &str, path: &str, body: Value) -> Result<(), String> {
    ureq::post(&format!("https://api.sowork.com/api/v1/{path}"))
        .set("authorization", &format!("Bearer {token}"))
        .set("source", "web")
        .set("version", CLIENT_VERSION)
        .timeout(Duration::from_secs(10))
        .send_json(body)
        .map(|_| ())
        .map_err(|e| format!("{path}: {e}"))
}

/// The REST half of waving at someone (the gaia emote goes over the socket).
pub fn send(token: &str, to: &str) {
    if let Err(e) = post(token, "waveToUser/create", json!({"type": "wave_to_user", "toUserId": to})) {
        eprintln!("wave: {e}");
    }
}

/// Dismiss every wave notification from `from` (what the official client does when you accept or close a wave).
pub fn clear(token: &str, uid: &str, from: &str) {
    let ids: Vec<String> = pending(token, uid).unwrap_or_default().into_iter().filter(|n| n.from == from).map(|n| n.id).collect();
    if !ids.is_empty() {
        if let Err(e) = post(token, "notifications/markAs", json!({"notificationIds": ids, "action": "remove"})) {
            eprintln!("wave: {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_a_wave_notification_from_a_firestore_answer() {
        let resp = json!([
            {"document": {"name": "projects/p/databases/(default)/documents/users_sys/U1/notifications/N1", "fields": {
                "type": {"stringValue": "wave-to-user"},
                "data": {"mapValue": {"fields": {"from": {"mapValue": {"fields": {"userId": {"stringValue": "U2"}, "userName": {"stringValue": "Kury"}}}}}}}}}},
            {"document": {"name": "x/notifications/N2", "fields": {"type": {"stringValue": "something-else"}}}},
            {"readTime": "2026-10-07T12:00:00Z"}
        ]);
        assert_eq!(parse_notes(&resp), vec![WaveNote { id: "N1".into(), from: "U2".into(), name: "Kury".into() }]);
    }
}
