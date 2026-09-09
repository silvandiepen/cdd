//! Ranking directory names against the typed filter.
//!
//! The ordering is deliberately predictable rather than clever: an exact or
//! prefix match must never rank below a weaker fuzzy match, so the tier is
//! compared before any score.

/// Match quality, best first. The discriminant is the sort key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tier {
    Exact = 0,
    PrefixSensitive = 1,
    PrefixInsensitive = 2,
    SubstringSensitive = 3,
    SubstringInsensitive = 4,
    Fuzzy = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Score {
    pub tier: Tier,
    /// Higher is better. Only compared within a tier.
    pub points: i32,
}

/// Score a candidate name, or `None` when it does not match at all.
///
/// An empty filter matches everything at the same tier, so the caller's
/// alphabetical tie-break decides the order.
pub fn score(name: &str, filter: &str) -> Option<Score> {
    if filter.is_empty() {
        return Some(Score {
            tier: Tier::Exact,
            points: 0,
        });
    }

    if name == filter {
        return Some(Score {
            tier: Tier::Exact,
            points: 0,
        });
    }

    let lower_name = name.to_lowercase();
    let lower_filter = filter.to_lowercase();

    if lower_name == lower_filter {
        return Some(Score {
            tier: Tier::Exact,
            points: -1,
        });
    }

    if name.starts_with(filter) {
        return Some(Score {
            tier: Tier::PrefixSensitive,
            points: 0,
        });
    }
    if lower_name.starts_with(&lower_filter) {
        return Some(Score {
            tier: Tier::PrefixInsensitive,
            points: 0,
        });
    }

    if let Some(at) = name.find(filter) {
        return Some(Score {
            tier: Tier::SubstringSensitive,
            points: -(at as i32),
        });
    }
    if let Some(at) = lower_name.find(&lower_filter) {
        return Some(Score {
            tier: Tier::SubstringInsensitive,
            points: -(at as i32),
        });
    }

    fuzzy(&lower_name, &lower_filter).map(|points| Score {
        tier: Tier::Fuzzy,
        points,
    })
}

/// Ordered subsequence match.
///
/// Every filter character must appear, in order. The score rewards matches that
/// start early and stay compact, so the visible result still looks related to
/// what was typed.
fn fuzzy(name: &str, filter: &str) -> Option<i32> {
    let mut points = 0i32;
    let mut haystack = name.char_indices();
    let mut previous_index: Option<usize> = None;

    for needle in filter.chars() {
        let (index, _) = haystack.by_ref().find(|(_, c)| *c == needle)?;
        points -= match previous_index {
            // A gap between matched characters weakens the match.
            Some(previous) => (index - previous - 1) as i32,
            // So does starting late in the name.
            None => index as i32,
        };
        previous_index = Some(index);
    }

    Some(points)
}

/// Order two scored candidates. Better matches sort first; among equally good
/// matches the shorter name wins, because it contains less that was not typed.
pub fn compare(a: (&Score, &str), b: (&Score, &str)) -> std::cmp::Ordering {
    let (a_score, a_name) = a;
    let (b_score, b_name) = b;

    a_score
        .tier
        .cmp(&b_score.tier)
        .then(b_score.points.cmp(&a_score.points))
        .then(a_name.chars().count().cmp(&b_name.chars().count()))
        .then_with(|| compare_names(a_name, b_name))
}

/// Plain alphabetical order, case-insensitive first so `Src` and `src` sit
/// together. This is the whole ordering when nothing has been typed yet: with
/// no query there is no such thing as a better match, and a directory listing
/// is expected to read like `ls`.
pub fn compare_names(a: &str, b: &str) -> std::cmp::Ordering {
    a.to_lowercase().cmp(&b.to_lowercase()).then(a.cmp(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tier(name: &str, filter: &str) -> Option<Tier> {
        score(name, filter).map(|s| s.tier)
    }

    /// Rank a set of names the way [`crate::session`] does.
    fn rank<'a>(names: &[&'a str], filter: &str) -> Vec<&'a str> {
        let mut scored: Vec<(Score, &str)> = names
            .iter()
            .filter_map(|n| score(n, filter).map(|s| (s, *n)))
            .collect();
        if filter.is_empty() {
            scored.sort_by(|a, b| compare_names(a.1, b.1));
        } else {
            scored.sort_by(|a, b| compare((&a.0, a.1), (&b.0, b.1)));
        }
        scored.into_iter().map(|(_, n)| n).collect()
    }

    #[test]
    fn tiers_are_assigned_in_the_specified_order() {
        assert_eq!(tier("src", "src"), Some(Tier::Exact));
        assert_eq!(tier("Src", "src"), Some(Tier::Exact));
        assert_eq!(tier("srcfoo", "src"), Some(Tier::PrefixSensitive));
        assert_eq!(tier("SrcFoo", "src"), Some(Tier::PrefixInsensitive));
        assert_eq!(tier("mysrcfoo", "src"), Some(Tier::SubstringSensitive));
        assert_eq!(tier("mySRCfoo", "src"), Some(Tier::SubstringInsensitive));
        assert_eq!(tier("soaring-crane", "src"), Some(Tier::Fuzzy));
        assert_eq!(tier("nope", "src"), None);
    }

    #[test]
    fn an_exact_match_always_beats_a_fuzzy_one() {
        assert_eq!(
            rank(&["soaring-crane", "src"], "src"),
            vec!["src", "soaring-crane"]
        );
    }

    #[test]
    fn a_prefix_match_always_beats_a_substring_match() {
        assert_eq!(rank(&["mysrc", "srcx"], "src"), vec!["srcx", "mysrc"]);
    }

    #[test]
    fn readme_example_orders_by_prefix_then_length() {
        assert_eq!(
            rank(&["tiko-media", "tiko", "tiko-plans", "tiko-talk"], "ti"),
            vec!["tiko", "tiko-talk", "tiko-media", "tiko-plans"]
        );
    }

    #[test]
    fn readme_example_ranks_prefix_above_substring() {
        assert_eq!(
            rank(&["imagekid", "image-tools", "simple-image"], "im"),
            vec!["imagekid", "image-tools", "simple-image"]
        );
    }

    #[test]
    fn an_empty_filter_lists_everything_alphabetically() {
        assert_eq!(rank(&["b", "a", "c"], ""), vec!["a", "b", "c"]);
        // Not by length: with nothing typed, a listing should read like `ls`.
        assert_eq!(
            rank(&["components", "src", "config"], ""),
            vec!["components", "config", "src"]
        );
    }

    #[test]
    fn fuzzy_prefers_compact_matches() {
        assert_eq!(tier("camp", "cmp"), Some(Tier::Fuzzy));
        assert_eq!(tier("c_m_p", "cmp"), Some(Tier::Fuzzy));
        assert_eq!(rank(&["c_m_p", "camp"], "cmp"), vec!["camp", "c_m_p"]);
    }

    #[test]
    fn every_extra_character_narrows_the_candidate_set() {
        let names = ["components", "composables", "config", "core", "content"];
        let mut previous = rank(&names, "").len();
        for filter in ["c", "co", "com", "comp", "compo"] {
            let now = rank(&names, filter).len();
            assert!(now <= previous, "{filter} widened the result set");
            previous = now;
        }
    }

    #[test]
    fn unicode_names_match_case_insensitively() {
        assert_eq!(tier("Ünïcode", "ünï"), Some(Tier::PrefixInsensitive));
        assert_eq!(tier("プロジェクト", "プロ"), Some(Tier::PrefixSensitive));
        assert_eq!(tier("emoji-📁-dir", "📁"), Some(Tier::SubstringSensitive));
    }
}
