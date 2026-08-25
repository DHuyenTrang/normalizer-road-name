use std::sync::OnceLock;
use unicode_normalization::UnicodeNormalization;

use crate::rules::{AbbreviateAction, RemoveAction, RuleSpec, RULES};
use crate::Mode;

#[derive(Clone, Copy, Debug)]
struct Match<'a> {
    rule: &'a RuleSpec,
    end: usize,
    is_alias: bool,
    attached: bool,
}

#[derive(Debug)]
struct CompiledRule {
    canonical: String,
    aliases: Vec<String>,
}

const MAX_PREFIX_CHARS: usize = 15;
static COMPILED_RULES: OnceLock<Vec<CompiledRule>> = OnceLock::new();

/// Normalizes the first recognized road-type prefix in `input`.
///
/// Matching is Unicode NFC-aware and case-insensitive. Text after the matched
/// prefix is copied from the original input, so its Unicode representation and
/// internal whitespace are retained.
///
/// # Examples
///
/// ```
/// use road_name_normalizer::{normalize, Mode};
///
/// assert_eq!(normalize("Đường Nguyễn Trãi", Mode::Abbreviate), "Đ. Nguyễn Trãi");
/// assert_eq!(normalize("Phố Huế", Mode::Remove), "Huế");
/// ```
#[must_use]
pub fn normalize(input: &str, mode: Mode) -> String {
    let Some(found) = find_match(input) else {
        return capitalize_first_letter(input);
    };

    let matched = &input[..found.end];
    let raw_remainder = &input[found.end..];
    let remainder = raw_remainder.trim_start_matches(char::is_whitespace);
    let separated = !raw_remainder.is_empty() && !found.attached;

    let output = match mode {
        Mode::Abbreviate => match found.rule.abbreviate {
            AbbreviateAction::Keep if !found.is_alias => input.to_owned(),
            AbbreviateAction::Keep => join(
                &capitalize_first_letter(found.rule.canonical),
                remainder,
                separated,
            ),
            AbbreviateAction::Replace(prefix) => join(prefix, remainder, separated),
        },
        Mode::Remove => match found.rule.remove {
            RemoveAction::Remove => remainder.to_owned(),
            RemoveAction::Keep if !found.is_alias => input.to_owned(),
            RemoveAction::Keep => join(
                &capitalize_first_letter(found.rule.canonical),
                remainder,
                separated,
            ),
            RemoveAction::Replace(prefix) => join(prefix, remainder, separated),
            RemoveAction::RemoveLeadingWord => {
                let kept = matched
                    .split_once(char::is_whitespace)
                    .map_or("", |(_, suffix)| {
                        suffix.trim_start_matches(char::is_whitespace)
                    });
                join(kept, remainder, separated)
            }
            RemoveAction::NormalizeCode(prefix) if found.is_alias => {
                join(prefix, remainder, separated)
            }
            RemoveAction::NormalizeCode(_) => input.to_owned(),
        },
    };

    capitalize_first_letter(&output)
}

fn find_match(input: &str) -> Option<Match<'_>> {
    let candidates = prefix_candidates(input);
    for (rule, compiled) in RULES.iter().zip(compiled_rules()) {
        if let Some(end) = match_form(&candidates, &compiled.canonical) {
            if valid_boundary(input, end, false) {
                return Some(Match {
                    rule,
                    end,
                    is_alias: false,
                    attached: false,
                });
            }
        }

        for alias in &compiled.aliases {
            if let Some(end) = match_form(&candidates, alias) {
                let boundary = input[end..].chars().next();
                let attached = rule.allow_attached_code
                    && boundary.is_some_and(|character| character.is_ascii_digit());
                if valid_boundary(input, end, attached) {
                    return Some(Match {
                        rule,
                        end,
                        is_alias: true,
                        attached,
                    });
                }
            }
        }
    }
    None
}

fn compiled_rules() -> &'static [CompiledRule] {
    COMPILED_RULES.get_or_init(|| {
        RULES
            .iter()
            .map(|rule| CompiledRule {
                canonical: normalized_key(rule.canonical),
                aliases: rule
                    .aliases
                    .iter()
                    .map(|alias| normalized_key(alias))
                    .collect(),
            })
            .collect()
    })
}

fn prefix_candidates(input: &str) -> Vec<(usize, String)> {
    let mut candidates = Vec::with_capacity(MAX_PREFIX_CHARS + 2);
    for end in input
        .char_indices()
        .map(|(index, character)| index + character.len_utf8())
    {
        let candidate = normalized_key(&input[..end]);
        if candidate.chars().count() > MAX_PREFIX_CHARS {
            break;
        }
        candidates.push((end, candidate));
    }
    candidates
}

fn match_form(candidates: &[(usize, String)], expected: &str) -> Option<usize> {
    candidates
        .iter()
        .find_map(|(end, candidate)| (candidate == expected).then_some(*end))
}

fn valid_boundary(input: &str, end: usize, attached: bool) -> bool {
    match input[end..].chars().next() {
        None => true,
        Some(character) if character.is_whitespace() => true,
        Some(character) if attached => character.is_ascii_digit(),
        Some(_) => false,
    }
}

fn normalized_key(value: &str) -> String {
    value.nfc().flat_map(char::to_lowercase).collect()
}

fn join(prefix: &str, remainder: &str, separated: bool) -> String {
    if prefix.is_empty() {
        return remainder.to_owned();
    }
    if remainder.is_empty() {
        return prefix.to_owned();
    }

    let separator = if separated { " " } else { "" };
    let mut output = String::with_capacity(prefix.len() + separator.len() + remainder.len());
    output.push_str(prefix);
    output.push_str(separator);
    output.push_str(remainder);
    output
}

fn capitalize_first_letter(input: &str) -> String {
    let Some((index, character)) = input
        .char_indices()
        .find(|(_, character)| character.is_alphabetic())
    else {
        return input.to_owned();
    };

    let uppercase: String = character.to_uppercase().collect();
    if uppercase.chars().eq(std::iter::once(character)) {
        return input.to_owned();
    }

    let mut output = String::with_capacity(input.len() - character.len_utf8() + uppercase.len());
    output.push_str(&input[..index]);
    output.push_str(&uppercase);
    output.push_str(&input[index + character.len_utf8()..]);
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Reverse;
    use std::collections::HashMap;
    use std::path::PathBuf;

    #[test]
    fn catalog_has_nineteen_valid_non_conflicting_rules() {
        assert_eq!(RULES.len(), 19);
        let mut owners = HashMap::new();

        for (rule_index, rule) in RULES.iter().enumerate() {
            assert!(!rule.canonical.is_empty());
            for form in std::iter::once(rule.canonical).chain(rule.aliases.iter().copied()) {
                assert!(!form.is_empty());
                let key = normalized_key(form);
                assert!(
                    key.chars().count() <= MAX_PREFIX_CHARS,
                    "form {form:?} exceeds matcher capacity"
                );
                assert!(
                    owners.insert(key.clone(), rule_index).is_none(),
                    "duplicate normalized form: {key}"
                );

                for earlier in &RULES[..rule_index] {
                    for earlier_form in
                        std::iter::once(earlier.canonical).chain(earlier.aliases.iter().copied())
                    {
                        let earlier_key = normalized_key(earlier_form);
                        assert!(
                            !key.starts_with(&earlier_key) || key == earlier_key,
                            "longer form {form:?} must precede {earlier_form:?}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn capitalization_preserves_everything_except_first_letter() {
        assert_eq!(capitalize_first_letter("  nguyễn  trãi"), "  Nguyễn  trãi");
        assert_eq!(capitalize_first_letter("--"), "--");
        assert_eq!(capitalize_first_letter(""), "");
    }

    #[test]
    fn every_rule_form_is_idempotent_when_abbreviated() {
        for rule in RULES {
            for form in std::iter::once(rule.canonical).chain(rule.aliases.iter().copied()) {
                let input = if rule.allow_attached_code && form != rule.canonical {
                    format!("{form}2A")
                } else {
                    format!("{form} Tên Riêng")
                };
                let once = normalize(&input, Mode::Abbreviate);
                assert_eq!(
                    normalize(&once, Mode::Abbreviate),
                    once,
                    "not idempotent for {input:?}"
                );
            }
        }
    }

    #[test]
    #[ignore = "offline audit of the full reference snapshot"]
    fn reference_snapshot_audit() {
        let path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("gofa_vietnam_real_road_names.csv");
        let mut reader = csv::Reader::from_path(path).expect("reference CSV must be readable");
        let mut total = 0_u64;
        let mut matched = 0_u64;
        let mut changed = [0_u64; 2];
        let mut empty = [0_u64; 2];
        let mut by_rule = [0_u64; 19];
        let mut unmatched_tokens: HashMap<String, u64> = HashMap::new();

        for row in reader.records() {
            let row = row.expect("reference CSV must be valid");
            let input = row.get(0).expect("road_name column is required");
            total += 1;
            if let Some(found) = find_match(input) {
                matched += 1;
                let index = RULES
                    .iter()
                    .position(|rule| std::ptr::eq(rule, found.rule))
                    .expect("matched rule belongs to catalog");
                by_rule[index] += 1;
            } else if let Some(token) = input.split_whitespace().next() {
                *unmatched_tokens.entry(normalized_key(token)).or_default() += 1;
            }

            for (index, mode) in [Mode::Abbreviate, Mode::Remove].into_iter().enumerate() {
                let output = normalize(input, mode);
                changed[index] += u64::from(output != input);
                empty[index] += u64::from(output.is_empty());
            }
        }

        assert_eq!(total, 78_615, "reference snapshot size changed");
        let mut tokens: Vec<_> = unmatched_tokens.into_iter().collect();
        tokens.sort_unstable_by_key(|entry| Reverse(entry.1));
        eprintln!(
            "total={total} matched={matched} unmatched={}",
            total - matched
        );
        eprintln!(
            "changed_abbreviate={} changed_remove={}",
            changed[0], changed[1]
        );
        eprintln!("empty_abbreviate={} empty_remove={}", empty[0], empty[1]);
        for (rule, count) in RULES.iter().zip(by_rule) {
            eprintln!("rule={:?} count={count}", rule.canonical);
        }
        eprintln!("top_unmatched_tokens={:?}", &tokens[..tokens.len().min(20)]);
    }
}
