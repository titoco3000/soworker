//! World model: floors, walls, zones, props; collision, path finding and zone detection.
//! Tile centres sit at odd multiples of G; tile pitch T = 2G. Walls are tile-edge segments.
use crate::mapobj::{Kind, MapObj, SetObj};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub const G: f32 = 105.468_75;
pub const T: f32 = 2.0 * G;
pub const N: u8 = 1;
pub const E: u8 = 2;
pub const S: u8 = 4;
pub const W: u8 = 8;
/// Zone half-extent = ZK x stored size, centred on x,y (fitted on the recording, docs/FINDINGS.md).
pub const ZK: f32 = 1.1;
const MARGIN: f32 = 28.0;

#[derive(Clone, Debug)]
pub struct Zone {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub sx: f32,
    pub sy: f32,
    pub meeting: bool,
    pub name: Option<String>,
}

#[derive(Default)]
pub struct World {
    objects: HashMap<String, MapObj>,
    pub floors: HashSet<(i32, i32)>,
    pub walls: HashMap<(i32, i32), u8>,
    pub soft: HashMap<(i32, i32), u8>,
    pub zones: Vec<Zone>,
    pub assets: Vec<(f32, f32)>,
    dirty: bool,
}

pub fn tile(v: f32) -> i32 {
    (v / T).floor() as i32
}

/// Walls that do not block walking: "NoCollider" variants and flat borders (<= 0.15 m).
/// The catalog has no collider flag; unknown wall ids block.
fn wall_table() -> &'static HashMap<String, (String, String)> {
    use std::sync::OnceLock;
    static TABLE: OnceLock<HashMap<String, (String, String)>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let raw: serde_json::Value = serde_json::from_str(include_str!("../data/walls.json")).unwrap_or_default();
        raw.as_object()
            .map(|m| {
                m.iter()
                    .map(|(k, v)| {
                        let s = |f: &str| v.get(f).and_then(|x| x.as_str()).unwrap_or("").to_string();
                        (k.clone(), (s("name"), s("height")))
                    })
                    .collect()
            })
            .unwrap_or_default()
    })
}

pub fn wall_blocks(asset_id: &str) -> bool {
    let Some((name, height)) = wall_table().get(asset_id) else { return true };
    if name.to_lowercase().replace(' ', "").contains("nocollider") {
        return false;
    }
    let h: String = height.chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
    !h.parse::<f32>().map_or(false, |h| h <= 0.15)
}

impl World {
    pub fn apply(&mut self, o: SetObj) {
        match o.data {
            Some(d) => self.objects.insert(o.id, d),
            None => self.objects.remove(&o.id),
        };
        self.dirty = true;
    }

    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    /// Recompute the derived tile sets after a batch of object changes. Returns true if anything changed.
    pub fn rebuild_if_dirty(&mut self) -> bool {
        if !self.dirty {
            return false;
        }
        self.dirty = false;
        self.floors.clear();
        self.walls.clear();
        self.soft.clear();
        self.zones.clear();
        self.assets.clear();
        for (id, o) in &self.objects {
            let key = (tile(o.x), tile(o.y));
            match o.kind {
                Kind::Floor => {
                    self.floors.insert(key);
                }
                Kind::Wall => {
                    let bit = match o.orientation {
                        0 => N,
                        2 => E,
                        4 => S,
                        6 => W,
                        _ => 0,
                    };
                    let m = if wall_blocks(&o.asset_id) { &mut self.walls } else { &mut self.soft };
                    *m.entry(key).or_insert(0) |= bit;
                }
                Kind::Zone => self.zones.push(Zone {
                    id: id.clone(),
                    x: o.x,
                    y: o.y,
                    sx: o.sx,
                    sy: o.sy,
                    meeting: o.interaction.as_deref() == Some("meeting"),
                    name: o.meeting_name.clone(),
                }),
                Kind::Asset => self.assets.push((o.x, o.y)),
            }
        }
        self.zones.sort_by(|a, b| a.id.cmp(&b.id));
        true
    }

    fn wall(&self, i: i32, j: i32) -> u8 {
        self.walls.get(&(i, j)).copied().unwrap_or(0)
    }

    /// True if a point may not be occupied: void, or within MARGIN of a solid wall edge.
    pub fn blocked(&self, x: f32, y: f32) -> bool {
        let (i, j) = (tile(x), tile(y));
        if !self.floors.contains(&(i, j)) {
            return true;
        }
        let (x0, y0) = (i as f32 * T, j as f32 * T);
        let here = self.wall(i, j);
        ((here & N != 0 || self.wall(i, j - 1) & S != 0) && y < y0 + MARGIN)
            || ((here & S != 0 || self.wall(i, j + 1) & N != 0) && y > y0 + T - MARGIN)
            || ((here & W != 0 || self.wall(i - 1, j) & E != 0) && x < x0 + MARGIN)
            || ((here & E != 0 || self.wall(i + 1, j) & W != 0) && x > x0 + T - MARGIN)
    }

    /// Move by (dx, dy) in small sub-steps, sliding along obstacles axis by axis.
    pub fn step(&self, pos: (f32, f32), dx: f32, dy: f32) -> (f32, f32) {
        let n = ((dx.hypot(dy) / 8.0).ceil() as i32).max(1);
        let (mut x, mut y) = pos;
        for _ in 0..n {
            if !self.blocked(x + dx / n as f32, y) {
                x += dx / n as f32;
            }
            if !self.blocked(x, y + dy / n as f32) {
                y += dy / n as f32;
            }
        }
        (x, y)
    }

    fn can_step(&self, i: i32, j: i32, di: i32, dj: i32) -> bool {
        let (ni, nj) = (i + di, j + dj);
        if !self.floors.contains(&(ni, nj)) {
            return false;
        }
        let (out, back) = match (di, dj) {
            (0, -1) => (N, S),
            (1, 0) => (E, W),
            (0, 1) => (S, N),
            (-1, 0) => (W, E),
            _ => return true,
        };
        self.wall(i, j) & out == 0 && self.wall(ni, nj) & back == 0
    }

    fn tile_path(&self, start: (i32, i32), goal: (i32, i32)) -> Option<Vec<(i32, i32)>> {
        let h = |p: (i32, i32)| (((p.0 - goal.0) as f32).hypot((p.1 - goal.1) as f32) * 1000.0) as u32;
        let mut open = BinaryHeap::new();
        let mut g: HashMap<(i32, i32), u32> = HashMap::from([(start, 0)]);
        let mut from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        open.push(Reverse((h(start), start)));
        let mut guard = 0;
        while let Some(Reverse((_, cur))) = open.pop() {
            guard += 1;
            if guard > 40_000 {
                return None;
            }
            if cur == goal {
                let mut out = vec![cur];
                let mut k = cur;
                while let Some(&p) = from.get(&k) {
                    out.push(p);
                    k = p;
                }
                out.reverse();
                return Some(out);
            }
            for di in -1..=1 {
                for dj in -1..=1 {
                    if (di == 0 && dj == 0) || !self.can_step(cur.0, cur.1, di, dj) {
                        continue;
                    }
                    if di != 0 && dj != 0 && !(self.can_step(cur.0, cur.1, di, 0) && self.can_step(cur.0, cur.1, 0, dj)) {
                        continue; // no corner cutting
                    }
                    let nxt = (cur.0 + di, cur.1 + dj);
                    let cost = g[&cur] + (((di * di + dj * dj) as f32).sqrt() * 1000.0) as u32;
                    if cost < g.get(&nxt).copied().unwrap_or(u32::MAX) {
                        g.insert(nxt, cost);
                        from.insert(nxt, cur);
                        open.push(Reverse((cost + h(nxt), nxt)));
                    }
                }
            }
        }
        None
    }

    fn clear(&self, a: (f32, f32), b: (f32, f32)) -> bool {
        let n = ((b.0 - a.0).hypot(b.1 - a.1) / 10.0).ceil() as i32;
        (1..=n).all(|k| {
            let t = k as f32 / n as f32;
            !self.blocked(a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t)
        })
    }

    /// A* over tiles, then string pulling so the route is a few straight segments.
    /// Returns the waypoints and the (possibly adjusted) goal.
    pub fn find_path(&self, from: (f32, f32), goal: (f32, f32)) -> Option<(Vec<(f32, f32)>, (f32, f32))> {
        let gt = (tile(goal.0), tile(goal.1));
        if !self.floors.contains(&gt) {
            return None;
        }
        let goal = if self.blocked(goal.0, goal.1) {
            ((gt.0 as f32 + 0.5) * T, (gt.1 as f32 + 0.5) * T)
        } else {
            goal
        };
        let tiles = self.tile_path((tile(from.0), tile(from.1)), gt)?;
        let mut pts: Vec<(f32, f32)> = tiles[1..tiles.len().saturating_sub(1).max(1)]
            .iter()
            .map(|&(i, j)| ((i as f32 + 0.5) * T, (j as f32 + 0.5) * T))
            .collect();
        if tiles.len() < 2 {
            pts.clear();
        }
        pts.push(goal);
        let (mut out, mut a, mut idx) = (Vec::new(), from, 0usize);
        while idx < pts.len() {
            let far = (idx + 1..pts.len()).rev().find(|&k| self.clear(a, pts[k])).unwrap_or(idx);
            out.push(pts[far]);
            a = pts[far];
            idx = far + 1;
        }
        Some((out, goal))
    }

    pub fn zone_at(&self, x: f32, y: f32) -> Option<&Zone> {
        let mut best: Option<(&Zone, f32)> = None;
        for z in self.zones.iter().filter(|z| z.meeting) {
            let (dx, dy) = ((x - z.x).abs() / (z.sx * ZK), (y - z.y).abs() / (z.sy * ZK));
            let d = dx.max(dy);
            if d <= 1.0 && best.map_or(true, |(_, bd)| d < bd) {
                best = Some((z, d));
            }
        }
        best.map(|(z, _)| z)
    }

    /// Fraction of floor tiles reachable on foot from a tile (diagnostics/tests).
    pub fn reachable_from(&self, start: (i32, i32)) -> usize {
        let mut seen: HashSet<(i32, i32)> = HashSet::from([start]);
        let mut q = vec![start];
        while let Some((i, j)) = q.pop() {
            for (di, dj) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
                let n = (i + di, j + dj);
                if !seen.contains(&n) && self.can_step(i, j, di, dj) {
                    seen.insert(n);
                    q.push(n);
                }
            }
        }
        seen.len()
    }
}
