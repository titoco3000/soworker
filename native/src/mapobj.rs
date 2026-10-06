//! Decoder for `PbSetMapObject` (protobuf) carried by gaia `b_set_obj`. See docs/gaia-protocol.md 3.9.
use serde_json::Value;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Zone,
    Asset,
    Floor,
    Wall,
}

#[derive(Clone, Debug)]
pub struct MapObj {
    pub kind: Kind,
    pub x: f32,
    pub y: f32,
    pub asset_id: String,
    /// 0 N, 1 NE, 2 E, 3 SE, 4 S, 5 SW, 6 W, 7 NW
    pub orientation: u8,
    pub sx: f32,
    pub sy: f32,
    pub interaction: Option<String>,
    pub meeting_name: Option<String>,
}

#[derive(Debug)]
pub struct SetObj {
    pub id: String,
    /// `None` means delete.
    pub data: Option<MapObj>,
}

fn varint(b: &[u8], i: &mut usize) -> Result<u64, String> {
    let (mut r, mut shift) = (0u64, 0u32);
    loop {
        let c = *b.get(*i).ok_or("truncated varint")?;
        *i += 1;
        r |= u64::from(c & 0x7f) << shift;
        if c & 0x80 == 0 {
            return Ok(r);
        }
        shift += 7;
        if shift > 63 {
            return Err("varint too long".into());
        }
    }
}

enum Field<'a> {
    Varint(u64),
    Bytes(&'a [u8]),
}

fn fields(b: &[u8]) -> Result<Vec<(u32, Field<'_>)>, String> {
    let (mut i, mut out) = (0usize, Vec::new());
    while i < b.len() {
        let k = varint(b, &mut i)?;
        let (no, wt) = ((k >> 3) as u32, k & 7);
        let f = match wt {
            0 => Field::Varint(varint(b, &mut i)?),
            1 => {
                let s = b.get(i..i + 8).ok_or("truncated fixed64")?;
                i += 8;
                Field::Bytes(s)
            }
            5 => {
                let s = b.get(i..i + 4).ok_or("truncated fixed32")?;
                i += 4;
                Field::Bytes(s)
            }
            2 => {
                let l = varint(b, &mut i)? as usize;
                let s = b.get(i..i + l).ok_or("truncated bytes")?;
                i += l;
                Field::Bytes(s)
            }
            _ => return Err(format!("wire type {wt}")),
        };
        out.push((no, f));
    }
    Ok(out)
}

fn f32le(b: &[u8]) -> f32 {
    b.get(..4).map_or(0.0, |s| f32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}

/// google.protobuf.Value
fn value(b: &[u8]) -> Value {
    for (no, f) in fields(b).unwrap_or_default() {
        match (no, f) {
            (2, Field::Bytes(v)) if v.len() == 8 => {
                return serde_json::json!(f64::from_le_bytes(v.try_into().unwrap()))
            }
            (3, Field::Bytes(v)) => return Value::String(String::from_utf8_lossy(v).into()),
            (4, Field::Varint(v)) => return Value::Bool(v != 0),
            (5, Field::Bytes(v)) => return structure(v),
            (6, Field::Bytes(v)) => {
                return Value::Array(
                    fields(v)
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|(n, f)| match (n, f) {
                            (1, Field::Bytes(x)) => Some(value(x)),
                            _ => None,
                        })
                        .collect(),
                )
            }
            _ => {}
        }
    }
    Value::Null
}

/// google.protobuf.Struct
fn structure(b: &[u8]) -> Value {
    let mut map = serde_json::Map::new();
    for (no, f) in fields(b).unwrap_or_default() {
        if let (1, Field::Bytes(entry)) = (no, f) {
            let (mut k, mut v) = (String::new(), Value::Null);
            for (n2, f2) in fields(entry).unwrap_or_default() {
                match (n2, f2) {
                    (1, Field::Bytes(s)) => k = String::from_utf8_lossy(s).into(),
                    (2, Field::Bytes(s)) => v = value(s),
                    _ => {}
                }
            }
            map.insert(k, v);
        }
    }
    Value::Object(map)
}

pub fn decode_set_map_object(buf: &[u8]) -> Result<SetObj, String> {
    let (mut id, mut data) = (String::new(), None);
    for (no, f) in fields(buf)? {
        match (no, f) {
            (1, Field::Bytes(v)) => id = String::from_utf8_lossy(v).into(),
            (2, Field::Bytes(v)) => {
                let mut o = MapObj {
                    kind: Kind::Zone,
                    x: 0.0,
                    y: 0.0,
                    asset_id: String::new(),
                    orientation: 0,
                    sx: 1.0,
                    sy: 1.0,
                    interaction: None,
                    meeting_name: None,
                };
                for (n, f) in fields(v)? {
                    match (n, f) {
                        (1, Field::Varint(t)) => {
                            o.kind = match t {
                                1 => Kind::Asset,
                                2 => Kind::Floor,
                                3 => Kind::Wall,
                                _ => Kind::Zone,
                            }
                        }
                        (2, Field::Bytes(s)) => o.x = f32le(s),
                        (3, Field::Bytes(s)) => o.y = f32le(s),
                        (4, Field::Bytes(s)) => o.asset_id = String::from_utf8_lossy(s).into(),
                        (5, Field::Varint(t)) => o.orientation = t as u8,
                        (6, Field::Bytes(s)) => o.sx = f32le(s),
                        (7, Field::Bytes(s)) => o.sy = f32le(s),
                        (9, Field::Bytes(s)) => {
                            let st = structure(s);
                            o.interaction = st.get("type").and_then(Value::as_str).map(String::from);
                            o.meeting_name = st
                                .pointer("/data/meeting/name")
                                .and_then(Value::as_str)
                                .map(String::from);
                        }
                        _ => {}
                    }
                }
                data = Some(o);
            }
            _ => {}
        }
    }
    Ok(SetObj { id, data })
}
