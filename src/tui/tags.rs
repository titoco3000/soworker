//! Short tags that tell people apart on the map: built from prefixes of the name's words, a capital starting each
//! word ("G", "Ga", "GB", "GaB"). Every tag is distinct; the most constrained names pick first, each taking its
//! shortest tag still free.

use std::collections::{HashMap, HashSet};

/// At most this many words of a name are used: the first ones and the last.
const MAX_WORDS: usize = 4;

/// The tag of every person, by id.
pub fn assign(people: &[(String, String)]) -> HashMap<String, String> {
    let mut order: Vec<(&String, Vec<Vec<char>>)> = people.iter().map(|(id, name)| (id, words(name, id))).collect();
    order.sort_by(|a, b| count(&a.1).cmp(&count(&b.1)).then_with(|| a.1.cmp(&b.1)).then_with(|| a.0.cmp(b.0)));
    let mut taken = HashSet::new();
    let mut out = HashMap::new();
    for (id, w) in order {
        let tag = pick(&w, &taken);
        taken.insert(tag.clone());
        out.insert(id.clone(), tag);
    }
    out
}

/// The name's words, letters and digits only, each starting with a capital and the rest in lower case.
fn words(name: &str, id: &str) -> Vec<Vec<char>> {
    let split = |s: &str| -> Vec<Vec<char>> {
        s.split_whitespace()
            .map(|w| {
                w.chars()
                    .filter(|c| c.is_alphanumeric())
                    .enumerate()
                    .flat_map(|(i, c)| if i == 0 { c.to_uppercase().collect::<Vec<_>>() } else { c.to_lowercase().collect() })
                    .collect::<Vec<char>>()
            })
            .filter(|w| !w.is_empty())
            .collect()
    };
    let mut w = split(name);
    if w.is_empty() {
        w = split(id);
    }
    if w.is_empty() {
        w = vec![vec!['?']];
    }
    if w.len() > MAX_WORDS {
        let last = w.pop().unwrap_or_default();
        w.truncate(MAX_WORDS - 1);
        w.push(last);
    }
    w
}

/// How many tags a name can make: fewer means it picks earlier.
fn count(w: &[Vec<char>]) -> u64 {
    w.iter().enumerate().map(|(i, w)| w.len() as u64 + u64::from(i > 0)).product()
}

/// The shortest tag still free; among equally short ones, the most even split, then the longer first words
/// ("GB" before "Gu", "GaB" before "GBo"). If every tag is taken (same names), the shortest one with a number.
fn pick(w: &[Vec<char>], taken: &HashSet<String>) -> String {
    let total: usize = w.iter().map(Vec::len).sum();
    for len in 1..=total {
        let mut splits = Vec::new();
        splits_of(w, len, &mut vec![], &mut splits);
        splits.sort_by(|a, b| a.iter().max().cmp(&b.iter().max()).then_with(|| b.cmp(a)));
        for k in splits {
            let tag: String = w.iter().zip(&k).flat_map(|(w, &n)| w[..n].iter()).collect();
            if !taken.contains(&tag) {
                return tag;
            }
        }
    }
    let first: String = w[0][..1].iter().collect();
    (2..).map(|n| format!("{first}{n}")).find(|t| !taken.contains(t)).unwrap_or(first)
}

/// Every way to take `len` letters as word prefixes: at least one from the first word, any (or none) from the rest.
fn splits_of(w: &[Vec<char>], len: usize, cur: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
    let i = cur.len();
    if i == w.len() {
        if len == 0 {
            out.push(cur.clone());
        }
        return;
    }
    let rest: usize = w[i + 1..].iter().map(Vec::len).sum();
    let lo = usize::from(i == 0).max(len.saturating_sub(rest));
    for n in lo..=w[i].len().min(len) {
        cur.push(n);
        splits_of(w, len - n, cur, out);
        cur.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tags(names: &[&str]) -> Vec<String> {
        let people: Vec<(String, String)> = names.iter().enumerate().map(|(i, n)| (i.to_string(), n.to_string())).collect();
        let t = assign(&people);
        (0..names.len()).map(|i| t[&i.to_string()].clone()).collect()
    }

    #[test]
    fn grows_only_as_far_as_needed() {
        assert_eq!(tags(&["Guilherme Boulos"]), ["G"]);
        assert_eq!(tags(&["Guilherme Boulos", "Gabriel"]), ["GB", "G"]);
        assert_eq!(tags(&["Guilherme Boulos", "Gabriel", "Gabrielle"]), ["GB", "G", "Ga"]);
        assert_eq!(tags(&["Guilherme Boulos", "Gabriel", "Gabrielle", "Gabrielle Boulos"]), ["Gu", "G", "Ga", "GB"]);
    }

    #[test]
    fn same_names_get_numbers() {
        assert_eq!(tags(&["Al", "Al", "Al"]), ["A", "Al", "A2"]);
    }

    #[test]
    fn odd_names() {
        assert_eq!(tags(&["", "  ", "jean-luc picard"]), ["0", "1", "J"]);
        assert_eq!(tags(&["Ana", "Ana Maria Clara Sofia Souza"]), ["A", "AM"]);
    }
}
