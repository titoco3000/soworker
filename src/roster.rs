//! The office's member list, online and offline (docs/members-roster.md): Firestore documents
//! `group_membermaps/<groupId>_<shard>`, read over REST with the signed-in user's ID token.
use crate::proto::Net;
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

/// Fetch once on a background thread and report through `tx` (the socket thread forwards it to the window).
pub fn fetch_async(token: String, group_id: String, tx: std::sync::mpsc::Sender<Net>) {
    std::thread::spawn(move || {
        let _ = match fetch(&token, &group_id) {
            Ok(m) => tx.send(Net::Roster(m)),
            Err(e) => {
                eprintln!("{e}");
                Ok(())
            }
        };
    });
}

#[cfg(test)]
mod tests {
    use super::*;

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
