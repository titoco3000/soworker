//! The office map as Minecraft blocks: which tile gets grass, cobblestone, a fence or a barrier, and how SoWork
//! positions map to block positions. Pure functions over `World`; the mod only places what this says.
//!
//! One tile is SCALE × SCALE blocks. Block x = SCALE × (tile i - origin i) + k, block z likewise, where the origin is
//! the smallest floor tile index on each axis, so the map starts at block (0, 0). SoWork's E is +x and S is +z.
use crate::world::{World, E, N, S, T, W};
use std::collections::{BTreeMap, BTreeSet, HashSet};

/// Blocks per tile, on each axis.
pub const SCALE: i32 = 2;
/// One block in SoWork units.
pub const BLOCK: f32 = T / SCALE as f32;

/// Meeting room floors, most wanted first: each room gets the first one no room it touches has already got.
pub const MATERIALS: &[&str] = &[
    "cobblestone",
    "stone_bricks",
    "polished_andesite",
    "mossy_cobblestone",
    "bricks",
    "polished_granite",
    "smooth_stone",
    "deepslate_tiles",
    "polished_diorite",
    "mud_bricks",
    "spruce_planks",
    "oak_planks",
];

/// The SCALE × SCALE blocks of tile (i, j) (already relative to the origin).
fn blocks_of((i, j): (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    (0..SCALE).flat_map(move |a| (0..SCALE).map(move |b| (i * SCALE + a, j * SCALE + b)))
}

/// A meeting zone in block coordinates: the bounding box (inclusive) of the blocks of the meeting tiles in it.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneBox {
    pub id: String,
    pub name: String,
    pub i0: i32,
    pub j0: i32,
    pub i1: i32,
    pub j1: i32,
    /// Index into MATERIALS.
    pub material: usize,
}

/// Everything the mod needs to build the map. All tile lists are in block coordinates and sorted.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Map {
    /// Smallest floor tile (i, j): block (0, 0).
    pub origin: (i32, i32),
    /// Floor tiles that are not meeting tiles (grass).
    pub floors: Vec<(i32, i32)>,
    /// Floor blocks of tiles whose centre is in a meeting zone, with the zone's material (index into MATERIALS).
    pub meeting: Vec<(i32, i32, usize)>,
    /// The blocks along a solid wall: the last row or column of blocks of a tile with a wall on its E or S edge (oak
    /// fence one block up).
    pub fences: Vec<(i32, i32)>,
    /// The blocks of the ring of tiles around the floor that are not floor, except fences (barriers, so nobody walks
    /// off the map).
    pub barriers: Vec<(i32, i32)>,
    pub zones: Vec<ZoneBox>,
}

/// The block origin of a world: the smallest floor tile index on each axis ((0, 0) for an empty map).
pub fn origin(w: &World) -> (i32, i32) {
    let i = w.floors.iter().map(|t| t.0).min().unwrap_or(0);
    let j = w.floors.iter().map(|t| t.1).min().unwrap_or(0);
    (i, j)
}

/// The tile centre of (i, j) in SoWork units.
fn centre(i: i32, j: i32) -> (f32, f32) {
    ((i as f32 + 0.5) * T, (j as f32 + 0.5) * T)
}

/// Build the block map of a world.
pub fn build(w: &World) -> Map {
    let o = origin(w);
    let b = |(i, j): (i32, i32)| (i - o.0, j - o.1);
    let mut floors = vec![];
    let mut meeting = vec![];
    let mut boxes: BTreeMap<String, (i32, i32, i32, i32)> = BTreeMap::new();
    let zone_of = |(i, j): (i32, i32)| {
        let (cx, cy) = centre(i, j);
        w.floors.contains(&(i, j)).then(|| w.zone_at(cx, cy)).flatten().map(|z| z.id.as_str())
    };
    let material = materials(w, &zone_of);
    for &(i, j) in &w.floors {
        let (cx, cy) = centre(i, j);
        let (bi, bj) = b((i, j));
        match w.zone_at(cx, cy) {
            Some(z) => {
                let k = material.get(z.id.as_str()).copied().unwrap_or(0);
                meeting.extend(blocks_of((bi, bj)).map(|(x, z)| (x, z, k)));
                let (x0, z0, x1, z1) = (bi * SCALE, bj * SCALE, bi * SCALE + SCALE - 1, bj * SCALE + SCALE - 1);
                let e = boxes.entry(z.id.clone()).or_insert((x0, z0, x1, z1));
                *e = (e.0.min(x0), e.1.min(z0), e.2.max(x1), e.3.max(z1));
            }
            None => floors.extend(blocks_of((bi, bj))),
        }
    }
    // a wall on the E edge of a tile: its last column of blocks; on the S edge: its last row
    let mut fence_set: BTreeSet<(i32, i32)> = BTreeSet::new();
    for (&(i, j), &bits) in &w.walls {
        let (bi, bj) = b((i, j));
        let edge = |bits: u8, (bi, bj): (i32, i32), out: &mut BTreeSet<(i32, i32)>| {
            for k in 0..SCALE {
                if bits & E != 0 {
                    out.insert((bi * SCALE + SCALE - 1, bj * SCALE + k));
                }
                if bits & S != 0 {
                    out.insert((bi * SCALE + k, bj * SCALE + SCALE - 1));
                }
            }
        };
        edge(bits & (E | S), (bi, bj), &mut fence_set);
        if bits & W != 0 {
            edge(E, (bi - 1, bj), &mut fence_set);
        }
        if bits & N != 0 {
            edge(S, (bi, bj - 1), &mut fence_set);
        }
    }
    let mut ring: HashSet<(i32, i32)> = HashSet::new();
    for &(i, j) in &w.floors {
        for di in -1..=1 {
            for dj in -1..=1 {
                let n = (i + di, j + dj);
                if !w.floors.contains(&n) {
                    ring.extend(blocks_of(b(n)).filter(|k| !fence_set.contains(k)));
                }
            }
        }
    }
    close_corners(&mut fence_set, &ring);
    let fences: Vec<(i32, i32)> = fence_set.iter().copied().collect();
    let mut barriers: Vec<(i32, i32)> = ring.into_iter().collect();
    floors.sort_unstable();
    meeting.sort_unstable();
    barriers.sort_unstable();
    let zones = boxes
        .into_iter()
        .map(|(id, (i0, j0, i1, j1))| {
            let name = w.zones.iter().find(|z| z.id == id).and_then(|z| z.name.clone()).unwrap_or_else(|| "Meeting".into());
            let material = material.get(id.as_str()).copied().unwrap_or(0);
            ZoneBox { id, name, i0, j0, i1, j1, material }
        })
        .collect();
    Map { origin: o, floors, meeting, fences, barriers, zones }
}

/// Two walls meeting at a corner can come out as fence blocks touching only diagonally (a wall on the W edge of a tile
/// is the last column of the tile to its west, one on its N edge the last row of the tile to its north): fence posts
/// that do not join leave a gap a player walks through, while SoWork's corner is closed. Wherever two solid blocks
/// (fences or barriers) touch only at a corner, a fence in one of the two open blocks closes it.
fn close_corners(fences: &mut BTreeSet<(i32, i32)>, barriers: &HashSet<(i32, i32)>) {
    loop {
        let solid = |b: &(i32, i32)| fences.contains(b) || barriers.contains(b);
        let mut add = None;
        'find: for &(x, z) in fences.iter().chain(barriers.iter()) {
            for dz in [-1, 1] {
                let (d, o1, o2) = ((x + 1, z + dz), (x + 1, z), (x, z + dz));
                if solid(&d) && !solid(&o1) && !solid(&o2) {
                    add = Some(o1.min(o2));
                    break 'find;
                }
            }
        }
        match add {
            Some(b) => {
                fences.insert(b);
            }
            None => return,
        }
    }
}

/// Each meeting zone's floor material: zones in id order take the first material that no zone touching them (a tile of
/// one next to a tile of the other, corners included) has already got.
fn materials<'a>(w: &'a World, zone_of: &impl Fn((i32, i32)) -> Option<&'a str>) -> BTreeMap<&'a str, usize> {
    let mut touch: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for &(i, j) in &w.floors {
        let Some(z) = zone_of((i, j)) else { continue };
        let near = touch.entry(z).or_default();
        for di in -1..=1 {
            for dj in -1..=1 {
                if let Some(o) = zone_of((i + di, j + dj)).filter(|&o| o != z) {
                    near.insert(o);
                }
            }
        }
    }
    let mut out: BTreeMap<&str, usize> = BTreeMap::new();
    for (z, near) in &touch {
        let taken: BTreeSet<usize> = near.iter().filter_map(|o| out.get(o).copied()).collect();
        let k = (0..MATERIALS.len()).find(|k| !taken.contains(k)).unwrap_or(out.len() % MATERIALS.len());
        out.insert(z, k);
    }
    out
}

/// A route for a minecart from block `from` to block `to`: four-way steps over floor blocks (not fences), shortest
/// first. None when there is none.
pub fn rail_route(m: &Map, from: (i32, i32), to: (i32, i32)) -> Option<Vec<(i32, i32)>> {
    let fences: HashSet<(i32, i32)> = m.fences.iter().copied().collect();
    let open: HashSet<(i32, i32)> =
        m.floors.iter().copied().chain(m.meeting.iter().map(|&(x, z, _)| (x, z))).filter(|b| !fences.contains(b)).collect();
    if !open.contains(&from) || !open.contains(&to) {
        return None;
    }
    let mut prev: std::collections::HashMap<(i32, i32), (i32, i32)> = std::collections::HashMap::from([(from, from)]);
    let mut queue = std::collections::VecDeque::from([from]);
    while let Some(c) = queue.pop_front() {
        if c == to {
            let mut out = vec![c];
            let mut k = c;
            while k != from {
                k = prev[&k];
                out.push(k);
            }
            out.reverse();
            return Some(out);
        }
        for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let n = (c.0 + d.0, c.1 + d.1);
            if open.contains(&n) && !prev.contains_key(&n) {
                prev.insert(n, c);
                queue.push_back(n);
            }
        }
    }
    None
}

/// A SoWork point as a fractional block position (x, z).
pub fn to_block(origin: (i32, i32), x: f32, y: f32) -> (f64, f64) {
    let s = SCALE as f64;
    ((x as f64 / T as f64 - origin.0 as f64) * s, (y as f64 / T as f64 - origin.1 as f64) * s)
}

/// A fractional block position (x, z) as a SoWork point.
pub fn from_block(origin: (i32, i32), bx: f64, bz: f64) -> (f32, f32) {
    let s = SCALE as f64;
    (((bx / s + origin.0 as f64) * T as f64) as f32, ((bz / s + origin.1 as f64) * T as f64) as f32)
}

/// The block (integer) a SoWork point is on.
pub fn block_of(origin: (i32, i32), x: f32, y: f32) -> (i32, i32) {
    let (bx, bz) = to_block(origin, x, y);
    (bx.floor() as i32, bz.floor() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::Zone;

    fn world(floors: impl IntoIterator<Item = (i32, i32)>) -> World {
        let mut w = World::default();
        w.floors = floors.into_iter().collect();
        w
    }

    #[test]
    fn origin_is_the_smallest_floor_tile() {
        let w = world([(5, 9), (3, 12), (7, 8)]);
        assert_eq!(origin(&w), (3, 8));
        let m = build(&w);
        assert_eq!(m.floors.len(), 12, "each tile is 2x2 blocks");
        for b in [(0, 8), (1, 9), (4, 2), (5, 3), (8, 0), (9, 1)] {
            assert!(m.floors.contains(&b), "{b:?}");
        }
        assert_eq!(origin(&World::default()), (0, 0));
    }

    #[test]
    fn west_wall_on_the_map_edge_puts_the_fence_outside_and_no_barrier_there() {
        let mut w = world([(0, 0), (1, 0)]);
        w.walls.insert((0, 0), W);
        let m = build(&w);
        assert_eq!(m.fences, [(-1, 0), (-1, 1)], "the last column of the tile west of the map");
        assert!(!m.barriers.contains(&(-1, 0)) && !m.barriers.contains(&(-1, 1)));
        assert!(m.barriers.contains(&(-2, 0)) && m.barriers.contains(&(-1, 2)) && m.barriers.contains(&(2, -1)));
    }

    #[test]
    fn a_wall_is_the_row_or_column_of_blocks_on_its_edge() {
        let mut w = world((0..3).flat_map(|i| (0..3).map(move |j| (i, j))));
        w.walls.insert((1, 1), E); // last column of (1, 1)
        w.walls.insert((0, 0), S); // last row of (0, 0)
        w.walls.insert((2, 2), W); // last column of (1, 2)
        w.walls.insert((2, 1), N); // last row of (2, 0)
        w.soft.insert((0, 2), E); // soft walls get nothing
        // (3, 1): the E wall of (1, 1) and the N wall of (2, 1) meet at a corner (see close_corners)
        assert_eq!(build(&w).fences, [(0, 1), (1, 1), (3, 1), (3, 2), (3, 3), (3, 4), (3, 5), (4, 1), (5, 1)]);
    }

    #[test]
    fn touching_meeting_rooms_get_different_floors() {
        let mut w = world((0..12).flat_map(|i| (0..4).map(move |j| (i, j))));
        let room = |id: &str, i: i32| {
            let (x, y) = centre(i, 1);
            Zone { id: id.into(), x, y, sx: 100.0, sy: 100.0, meeting: true, name: None } // one tile each
        };
        // a and b touch, c and d touch; the two pairs are apart, so each pair starts again at the top of the list
        w.zones.push(room("a", 1));
        w.zones.push(room("b", 2));
        w.zones.push(room("c", 6));
        w.zones.push(room("d", 7));
        let m = build(&w);
        let k: BTreeMap<_, _> = m.zones.iter().map(|z| (z.id.as_str(), z.material)).collect();
        assert_eq!(k, BTreeMap::from([("a", 0), ("b", 1), ("c", 0), ("d", 1)]));
        assert!(m.meeting.contains(&(4, 2, 1)) && m.meeting.contains(&(2, 2, 0)));
    }

    #[test]
    fn rail_routes_go_round_fences() {
        let mut w = world((0..3).flat_map(|i| (0..3).map(move |j| (i, j))));
        w.walls.insert((0, 0), E);
        w.walls.insert((0, 1), E);
        let m = build(&w);
        let r = rail_route(&m, (0, 0), (2, 0)).unwrap();
        assert_eq!((r[0], *r.last().unwrap()), ((0, 0), (2, 0)));
        assert!(r.iter().all(|b| !m.fences.contains(b)));
        assert!(r.windows(2).all(|p| (p[0].0 - p[1].0).abs() + (p[0].1 - p[1].1).abs() == 1));
        assert!(r.iter().any(|b| b.1 >= 4), "round the wall's south end");
        assert_eq!(rail_route(&m, (0, 0), (1, 0)), None, "a fence block is not a stop");
    }

    #[test]
    fn walls_meeting_at_a_corner_leave_no_gap() {
        let mut w = world((0..3).flat_map(|i| (0..3).map(move |j| (i, j))));
        w.walls.insert((1, 1), W | N); // column x = 1 (z 2, 3) and row z = 1 (x 2, 3): they touch only at a corner
        let m = build(&w);
        assert!(m.fences.contains(&(1, 1)), "{:?}", m.fences);
        for &(x, z) in &m.fences {
            for dz in [-1, 1] {
                let f = |b| m.fences.contains(&b) || m.barriers.contains(&b);
                assert!(!(f((x + 1, z + dz)) && !f((x + 1, z)) && !f((x, z + dz))), "gap at {:?}", (x, z));
            }
        }
    }

    #[test]
    fn barriers_ring_the_floor() {
        let m = build(&world([(0, 0)]));
        assert_eq!(m.barriers.len(), 32, "8 tiles of 4 blocks");
        assert!(!m.barriers.contains(&(0, 0)) && !m.barriers.contains(&(1, 1)));
        let m = build(&world((0..3).flat_map(|i| (0..3).map(move |j| (i, j)))));
        assert_eq!(m.barriers.len(), 64, "5x5 minus the 3x3 floor, in blocks");
    }

    #[test]
    fn meeting_tiles_are_the_floor_tiles_whose_centre_is_in_a_meeting_zone() {
        let mut w = world((0..10).flat_map(|i| (0..6).map(move |j| (i, j))));
        // centred on tile (5, 2)'s centre, half-extent 1.1 * 200 = 220 (just over one tile): tiles 4..=6, 1..=3
        let (cx, cy) = centre(5, 2);
        w.zones.push(Zone { id: "z1".into(), x: cx, y: cy, sx: 200.0, sy: 200.0, meeting: true, name: Some("Room".into()) });
        w.zones.push(Zone { id: "z2".into(), x: centre(1, 1).0, y: centre(1, 1).1, sx: 200.0, sy: 200.0, meeting: false, name: None });
        let m = build(&w);
        let want: Vec<_> = (8..=13).flat_map(|i| (2..=7).map(move |j| (i, j, 0))).collect();
        assert_eq!(m.meeting, want);
        assert_eq!(m.floors.len() + m.meeting.len(), 240);
        assert!(m.floors.iter().all(|&(x, z)| !m.meeting.contains(&(x, z, 0))));
        assert_eq!(m.zones, [ZoneBox { id: "z1".into(), name: "Room".into(), i0: 8, j0: 2, i1: 13, j1: 7, material: 0 }]);
    }

    #[test]
    fn block_conversion_round_trips_and_keeps_the_fraction() {
        let o = (-3, 7);
        let (x, y) = (2.25 * T, 9.5 * T);
        let (bx, bz) = to_block(o, x, y);
        assert!((bx - 10.5).abs() < 1e-4 && (bz - 5.0).abs() < 1e-4);
        let (x2, y2) = from_block(o, bx, bz);
        assert!((x2 - x).abs() < 0.01 && (y2 - y).abs() < 0.01);
        assert_eq!(block_of(o, x, y), (10, 5));
        assert_eq!(block_of(o, -0.1, 7.0 * T), (5, 0), "tile -1 on x: its east block");
    }
}
