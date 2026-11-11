//! The office's member list, online and offline (docs/members-roster.md). Live: the chat socket's `members` query,
//! what the web app's people list shows (`Live`). The Firestore documents `group_membermaps/<groupId>_<shard>` (`fetch`)
//! are no longer kept current by SoWork (hours stale), so only `soworker --roster` still reads them.
use serde_json::Value;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Member {
    pub id: String,
    pub name: String,
    pub online: bool,
    /// `busy` (or `away`): shown as away.
    pub status: String,
    pub mobile: bool,
    /// Last time the member was online, milliseconds since the Unix epoch.
    pub last_online: Option<i64>,
    /// The meeting zone an online member stands in, when the roster knows it.
    pub zone: Option<String>,
}

const PROJECT: &str = "sage-learning-prod";
const MAX_SHARDS: u32 = 32;

fn string_of(v: &Value) -> Option<&str> {
    v.get("stringValue").or_else(|| v.get("timestampValue")).and_then(Value::as_str)
}

/// Members of one `group_membermaps` document (Firestore REST JSON).
pub fn parse_doc(doc: &Value) -> Vec<Member> {
    let Some(users) = doc.pointer("/fields/users/mapValue/fields").and_then(Value::as_object) else { return vec![] };
    users
        .iter()
        .filter_map(|(id, u)| {
            let f = u.pointer("/mapValue/fields")?;
            let s = |k: &str| f.get(k).and_then(string_of).map(String::from);
            let b = |k: &str| f.get(k).and_then(|v| v.get("booleanValue")).and_then(Value::as_bool).unwrap_or(false);
            Some(Member {
                id: id.clone(),
                name: s("userName").unwrap_or_default(),
                online: b("isOnline"),
                status: s("status").unwrap_or_else(|| "online".into()),
                mobile: b("mobile"),
                last_online: s("lastOnline").and_then(|t| iso_to_ms(&t)),
                zone: s("meetingZoneId").filter(|z| !z.is_empty()),
            })
        })
        .collect()
}

/// `2026-10-07T11:45:52.402Z` -> epoch milliseconds (UTC only, which is all Firestore sends).
pub fn iso_to_ms(t: &str) -> Option<i64> {
    let (date, time) = t.trim_end_matches('Z').split_once('T')?;
    let mut d = date.split('-').map(|p| p.parse::<i64>().ok());
    let (y, m, day) = (d.next()??, d.next()??, d.next()??);
    let mut hms = time.split(':');
    let (h, mi) = (hms.next()?.parse::<i64>().ok()?, hms.next()?.parse::<i64>().ok()?);
    let sec = hms.next().unwrap_or("0").parse::<f64>().ok()?;
    // days since 1970-01-01 (Howard Hinnant's civil-from-days, inverted)
    let (y, m) = if m <= 2 { (y - 1, m + 9) } else { (y, m - 3) };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let doy = (153 * m + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146097 + doe - 719468;
    Some(((days * 86400 + h * 3600 + mi * 60) as f64 * 1000.0 + sec * 1000.0) as i64)
}

/// Read every shard (0, 1, 2, ... until one does not exist) and merge the members.
pub fn fetch(token: &str, group_id: &str) -> Result<Vec<Member>, String> {
    let mut out = Vec::new();
    for shard in 0..MAX_SHARDS {
        let url = format!("https://firestore.googleapis.com/v1/projects/{PROJECT}/databases/(default)/documents/group_membermaps/{group_id}_{shard}");
        match ureq::get(&url).set("authorization", &format!("Bearer {token}")).timeout(std::time::Duration::from_secs(10)).call() {
            Ok(r) => out.extend(parse_doc(&r.into_json::<Value>().map_err(|e| e.to_string())?)),
            Err(ureq::Error::Status(404, _)) if shard > 0 => break,
            Err(e) => return Err(format!("roster shard {shard}: {e}")),
        }
    }
    Ok(out)
}

/// Our subscription id for the member list on the chat socket.
const SUB_ID: &str = "soworker:members";

/// `query.subscribe` for every member, as the web app asks for its people list (sent after `connection.ready`).
pub fn subscribe_frame(group_id: &str) -> Value {
    serde_json::json!({"type": "query.subscribe", "data": {
        "sub_id": SUB_ID, "collection": "members", "group_id": group_id,
        "query": {"filter": {"include_apps": false}, "sort": ["availability", "last_seen", "name"], "limit": 5000},
        "time_zone": "UTC",
    }})
}

/// One row of `members.rows` / `members.changes`: `{user_id, name, last_seen_at, presence: {online, status, zone_id,
/// mobile_reachable, sessions: [{kind, state}], version, ...}}`. Returns the member and its presence version.
pub fn parse_row(r: &Value) -> Option<(Member, i64)> {
    let p = r.get("presence")?;
    let online = p["online"].as_bool().unwrap_or(false);
    let on_phone = p["sessions"].as_array().is_some_and(|s| s.iter().any(|s| s["kind"] == "mobile" && s["state"] != "background"));
    let m = Member {
        id: r["user_id"].as_str()?.to_string(),
        name: r["name"].as_str().unwrap_or("").to_string(),
        online,
        status: p["status"].as_str().unwrap_or("online").to_string(),
        mobile: p["mobile_reachable"].as_bool().unwrap_or(false) || on_phone,
        last_online: r["last_seen_at"].as_i64(),
        zone: p["zone_id"].as_str().filter(|z| online && !z.is_empty()).map(String::from),
    };
    Some((m, p["version"].as_i64().unwrap_or(0)))
}

/// The member list as the chat socket keeps it current: the full rows once, then upserts and removals.
#[derive(Default)]
pub struct Live {
    rows: std::collections::HashMap<String, (Member, i64)>,
}

impl Live {
    /// Take a chat socket frame; the whole list when it changed it (None for every other frame).
    pub fn frame(&mut self, text: &str) -> Option<Vec<Member>> {
        if !text.contains("\"members.") {
            return None;
        }
        let v: Value = serde_json::from_str(text).ok()?;
        let d = &v["data"];
        if d["sub_id"] != SUB_ID {
            return None;
        }
        let rows = |k: &str| d[k].as_array().into_iter().flatten().filter_map(parse_row).collect::<Vec<_>>();
        match v["type"].as_str()? {
            "members.rows" => self.rows = rows("rows").into_iter().map(|(m, ver)| (m.id.clone(), (m, ver))).collect(),
            "members.changes" => {
                for id in d["removed"].as_array().into_iter().flatten().filter_map(Value::as_str) {
                    self.rows.remove(id);
                }
                for (m, ver) in rows("upserted") {
                    // an older presence than the one we have keeps ours (the web app does the same)
                    if self.rows.get(&m.id).is_none_or(|(_, have)| *have <= ver) {
                        self.rows.insert(m.id.clone(), (m, ver));
                    }
                }
            }
            _ => return None,
        }
        let mut out: Vec<Member> = self.rows.values().map(|(m, _)| m.clone()).collect();
        out.sort_by(|a, b| a.id.cmp(&b.id));
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follows_the_live_member_list() {
        let row = |id: &str, online: bool, ver: i64| serde_json::json!({"user_id": id, "name": id, "last_seen_at": 5,
            "presence": {"online": online, "status": "online", "zone_id": if online { "Z" } else { "" }, "sessions": [], "version": ver}});
        let mut l = Live::default();
        let rows = serde_json::json!({"type": "members.rows", "data": {"sub_id": SUB_ID, "rows": [row("a", true, 1), row("b", false, 1)]}});
        let m = l.frame(&rows.to_string()).unwrap();
        assert_eq!(m.iter().map(|m| (m.id.as_str(), m.online, m.zone.as_deref())).collect::<Vec<_>>(), [("a", true, Some("Z")), ("b", false, None)]);
        let ch = serde_json::json!({"type": "members.changes", "data": {"sub_id": SUB_ID, "removed": ["b"], "upserted": [row("a", false, 3), row("c", true, 1)]}});
        let m = l.frame(&ch.to_string()).unwrap();
        assert_eq!(m.iter().map(|m| (m.id.as_str(), m.online)).collect::<Vec<_>>(), [("a", false), ("c", true)]);
        let stale = serde_json::json!({"type": "members.changes", "data": {"sub_id": SUB_ID, "removed": [], "upserted": [row("a", true, 2)]}});
        assert!(!l.frame(&stale.to_string()).unwrap()[0].online, "an older presence version is ignored");
        assert!(l.frame(r#"{"type":"message.new","data":{}}"#).is_none());
    }

    #[test]
    fn parses_a_membermaps_document() {
        let doc = serde_json::json!({"fields": {"users": {"mapValue": {"fields": {
            "u1": {"mapValue": {"fields": {
                "userName": {"stringValue": "Ana"}, "isOnline": {"booleanValue": false}, "status": {"stringValue": "busy"},
                "mobile": {"booleanValue": true}, "lastOnline": {"timestampValue": "2026-10-07T11:45:52.402Z"}}}},
            "u2": {"mapValue": {"fields": {
                "userName": {"stringValue": "Bo"}, "isOnline": {"booleanValue": true}, "meetingZoneId": {"stringValue": "Z1"}}}}
        }}}}});
        let mut m = parse_doc(&doc);
        m.sort_by(|a, b| a.id.cmp(&b.id));
        assert_eq!(m[0], Member { id: "u1".into(), name: "Ana".into(), online: false, status: "busy".into(), mobile: true, last_online: Some(1_791_373_552_402), zone: None });
        assert!(m[1].online && m[1].zone.as_deref() == Some("Z1") && m[1].status == "online");
    }

    #[test]
    fn iso_times_convert_to_epoch_milliseconds() {
        assert_eq!(iso_to_ms("1970-01-01T00:00:00Z"), Some(0));
        assert_eq!(iso_to_ms("2000-03-01T00:00:00.500Z"), Some(951_868_800_500));
        assert_eq!(iso_to_ms("not a time"), None);
    }
}
