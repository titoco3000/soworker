//! Benchmark mode (`tools/bench.mjs`): with `SOWORKER_BENCH` set, either client plays a fixed scenario by itself so
//! runs can be compared: wait in the office, then walk to a person (`join=<name>`) and, once in their call, turn the
//! microphone (`mic`) and camera (`cam`) on. What happened and when goes to the file named by
//! `SOWORKER_BENCH_EVENTS`, one `<unix ms> <event>` line each: `office` (the map is in and we are at our desk, as
//! the normal start-up walk takes us there), `walking`, `joined`.
//! The scenario's timing (how long to stay idle, when to quit) is the script's.

#[derive(Debug, Clone, PartialEq)]
pub struct Plan {
    /// Whose call to walk into (their name, any case); None = stay where we start.
    pub join: Option<String>,
    pub mic: bool,
    pub cam: bool,
    /// Walk to the person only once this file exists (the script creates it when the idle part is over).
    pub go_file: Option<String>,
    pub stage: Stage,
    /// The map's floor count and since when it is unchanged: the office is in once it holds for a second.
    pub map: (usize, f64),
    /// When we last set off towards the person (a walk that ends short is tried again).
    pub walked_at: f64,
    /// The call we were in when we set off (where we start may be a meeting already): only another one counts.
    pub from: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Stage {
    Loading,
    Office,
    Walking,
    Joined,
}

impl Plan {
    /// `join=Kury,mic,cam,go=/path`: the scenario from `SOWORKER_BENCH`.
    pub fn parse(spec: &str) -> Plan {
        let mut p = Plan { join: None, mic: false, cam: false, go_file: None, stage: Stage::Loading, map: (0, 0.0), walked_at: f64::MIN, from: None };
        for part in spec.split(',').map(str::trim) {
            match part.split_once('=') {
                Some(("join", name)) if !name.is_empty() => p.join = Some(name.to_string()),
                Some(("go", file)) if !file.is_empty() => p.go_file = Some(file.to_string()),
                None if part == "mic" => p.mic = true,
                None if part == "cam" => p.cam = true,
                _ => {}
            }
        }
        p
    }

    pub fn from_env() -> Option<Plan> {
        std::env::var("SOWORKER_BENCH").ok().filter(|s| !s.is_empty()).map(|s| Plan::parse(&s))
    }

    /// The idle part is over (or there is no signal to wait for).
    pub fn go(&self) -> bool {
        self.go_file.as_ref().map_or(true, |f| std::path::Path::new(f).exists())
    }
}

/// Notes an event for the benchmark script (and in the log).
pub fn mark(event: &str) {
    eprintln!("bench: {event}");
    let Some(path) = std::env::var_os("SOWORKER_BENCH_EVENTS") else { return };
    let ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis());
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(f, "{ms} {event}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_scenario_parses() {
        let p = Plan::parse("join=Kury, mic,go=/tmp/x");
        assert_eq!((p.join.as_deref(), p.mic, p.cam, p.go_file.as_deref()), (Some("Kury"), true, false, Some("/tmp/x")));
        assert_eq!(Plan::parse("").join, None);
        assert!(Plan::parse("cam").cam);
    }
}
