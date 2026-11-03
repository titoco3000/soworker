//! Who owns which desk (docs/hand-desk-photo.md): the Firestore document `group_desks/<groupId>`, read over REST with
//! the signed-in user's ID token. Where the desks are comes from gaia's `room` snapshot instead.
use crate::proto::Net;
use serde_json::Value;

/// One owned desk: the room definition it is in, the desk id, and the owner's user id.
#[derive(Clone, Debug, PartialEq)]
pub struct Desk {
    pub room: String,
    pub desk: String,
    pub owner: String,
}

/// `rooms.<roomDefinitionId>.<deskId>.userId` of a `group_desks` document (Firestore REST JSON).
pub fn parse_doc(doc: &Value) -> Vec<Desk> {
    let Some(rooms) = doc.pointer("/fields/rooms/mapValue/fields").and_then(Value::as_object) else { return vec![] };
    rooms
        .iter()
        .flat_map(|(room, desks)| {
            let desks = desks.pointer("/mapValue/fields").and_then(Value::as_object).into_iter().flatten();
            desks.filter_map(move |(desk, d)| {
                let owner = d.pointer("/mapValue/fields/userId/stringValue")?.as_str().filter(|o| !o.is_empty())?;
                Some(Desk { room: room.clone(), desk: desk.clone(), owner: owner.into() })
            })
        })
        .collect()
}

pub fn fetch(token: &str, group_id: &str) -> Result<Vec<Desk>, String> {
    let url = format!("https://firestore.googleapis.com/v1/projects/sage-learning-prod/databases/(default)/documents/group_desks/{group_id}");
    match ureq::get(&url).set("authorization", &format!("Bearer {token}")).timeout(std::time::Duration::from_secs(10)).call() {
        Ok(r) => Ok(parse_doc(&r.into_json::<Value>().map_err(|e| e.to_string())?)),
        Err(ureq::Error::Status(404, _)) => Ok(vec![]), // nobody in the office has a desk
        Err(e) => Err(format!("desks: {e}")),
    }
}

/// Fetch once on a background thread and report through `tx` (the socket thread forwards it to the window).
pub fn fetch_async(token: String, group_id: String, tx: std::sync::mpsc::Sender<Net>) {
    std::thread::spawn(move || match fetch(&token, &group_id) {
        Ok(d) => drop(tx.send(Net::DeskOwners(d))),
        Err(e) => eprintln!("{e}"),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_group_desks_document() {
        // the shape recorded on 2026-11-03
        let doc = serde_json::json!({"name": "projects/p/databases/(default)/documents/group_desks/G", "fields": {"rooms": {"mapValue": {"fields": {
            "yPsvwRdkb5LnGHfXFgfO": {"mapValue": {"fields": {
                "jQS-ZAKH5uYwHCSrSdkm": {"mapValue": {"fields": {"userId": {"stringValue": "me"}}}},
                "g5w_cnxMcAigxoIrTT3G": {"mapValue": {"fields": {"userId": {"stringValue": "kury"}}}},
                "empty": {"mapValue": {"fields": {}}}
            }}}
        }}}}});
        let mut d = parse_doc(&doc);
        d.sort_by(|a, b| a.owner.cmp(&b.owner));
        assert_eq!(d.len(), 2);
        assert_eq!(d[1], Desk { room: "yPsvwRdkb5LnGHfXFgfO".into(), desk: "jQS-ZAKH5uYwHCSrSdkm".into(), owner: "me".into() });
        assert!(parse_doc(&serde_json::json!({})).is_empty());
    }
}
