//! Replays the gaia frames of a recorder run (output/logs/<run>/events.jsonl) as `Net` events.
use crate::proto::{events, parse_packet, Net};
use base64::Engine;

pub fn load(run_dir: &str) -> Result<Vec<Net>, String> {
    let text = std::fs::read_to_string(format!("{run_dir}/events.jsonl")).map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for line in text.lines() {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else { continue };
        if v["k"] != "ws.in" || v["op"] != 2 {
            continue;
        }
        let Some(data) = v["data"].as_str() else { continue };
        let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(data) else { continue };
        if let Some((2, Some((name, payload)))) = parse_packet(&bytes) {
            out.extend(events(&name, &payload));
        }
    }
    Ok(out)
}

/// The recording user's id: the first `presence` event is always our own.
pub fn own_id(run_dir: &str) -> Result<String, String> {
    let text = std::fs::read_to_string(format!("{run_dir}/events.jsonl")).map_err(|e| e.to_string())?;
    for line in text.lines() {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else { continue };
        if v["k"] != "ws.in" || v["op"] != 2 {
            continue;
        }
        let Some(bytes) = v["data"].as_str().and_then(|d| base64::engine::general_purpose::STANDARD.decode(d).ok()) else { continue };
        if let Some((2, Some((name, p)))) = parse_packet(&bytes) {
            if name == "presence" {
                if let Some(id) = p.as_map().and_then(|m| m.iter().find(|(k, _)| k.as_str() == Some("userId"))).and_then(|(_, v)| v.as_str()) {
                    return Ok(id.to_string());
                }
            }
        }
    }
    Err("no presence event in the recording".into())
}
