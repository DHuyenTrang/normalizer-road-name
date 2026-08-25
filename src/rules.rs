#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AbbreviateAction {
    Keep,
    Replace(&'static str),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RemoveAction {
    Remove,
    Keep,
    Replace(&'static str),
    RemoveLeadingWord,
    NormalizeCode(&'static str),
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct RuleSpec {
    pub canonical: &'static str,
    pub aliases: &'static [&'static str],
    pub abbreviate: AbbreviateAction,
    pub remove: RemoveAction,
    pub allow_attached_code: bool,
}

const fn rule(
    canonical: &'static str,
    aliases: &'static [&'static str],
    abbreviate: AbbreviateAction,
    remove: RemoveAction,
    allow_attached_code: bool,
) -> RuleSpec {
    RuleSpec {
        canonical,
        aliases,
        abbreviate,
        remove,
        allow_attached_code,
    }
}

/// Rules are ordered so that a compound prefix is considered before a prefix
/// that can be its leading word (for example, `đường cao tốc` before `đường`).
pub(crate) static RULES: &[RuleSpec] = &[
    rule(
        "đường cao tốc",
        &[],
        AbbreviateAction::Replace("CT."),
        RemoveAction::Replace("CT."),
        false,
    ),
    rule(
        "đường vành đai",
        &[],
        AbbreviateAction::Replace("VĐ."),
        RemoveAction::RemoveLeadingWord,
        false,
    ),
    rule(
        "đường liên thôn",
        &[],
        AbbreviateAction::Keep,
        RemoveAction::Keep,
        false,
    ),
    rule(
        "đường liên xã",
        &[],
        AbbreviateAction::Keep,
        RemoveAction::Keep,
        false,
    ),
    rule(
        "đường tỉnh",
        &["ĐT", "ĐT.", "DT"],
        AbbreviateAction::Replace("ĐT."),
        RemoveAction::NormalizeCode("ĐT."),
        true,
    ),
    rule(
        "đường huyện",
        &[],
        AbbreviateAction::Replace("ĐH."),
        RemoveAction::Keep,
        false,
    ),
    rule(
        "quốc lộ",
        &["QL", "QL."],
        AbbreviateAction::Replace("QL."),
        RemoveAction::NormalizeCode("QL."),
        true,
    ),
    rule(
        "tỉnh lộ",
        &["TL", "TL."],
        AbbreviateAction::Replace("TL."),
        RemoveAction::NormalizeCode("TL."),
        true,
    ),
    rule(
        "huyện lộ",
        &["HL", "HL."],
        AbbreviateAction::Replace("HL."),
        RemoveAction::NormalizeCode("HL."),
        true,
    ),
    rule(
        "đại lộ",
        &[],
        AbbreviateAction::Replace("ĐL."),
        RemoveAction::Keep,
        false,
    ),
    rule(
        "cao tốc",
        &[],
        AbbreviateAction::Replace("CT."),
        RemoveAction::Replace("CT."),
        false,
    ),
    rule(
        "xa lộ",
        &[],
        AbbreviateAction::Replace("XL."),
        RemoveAction::Remove,
        false,
    ),
    rule(
        "vành đai",
        &["VĐ", "VĐ."],
        AbbreviateAction::Replace("VĐ."),
        RemoveAction::Remove,
        true,
    ),
    rule(
        "hẻm",
        &["hèm", "hem", "hẽm"],
        AbbreviateAction::Keep,
        RemoveAction::Keep,
        false,
    ),
    rule(
        "ngách",
        &[],
        AbbreviateAction::Replace("Ng."),
        RemoveAction::Keep,
        false,
    ),
    rule(
        "ngõ",
        &[],
        AbbreviateAction::Replace("Ng."),
        RemoveAction::Keep,
        false,
    ),
    rule(
        "kiệt",
        &[],
        AbbreviateAction::Keep,
        RemoveAction::Keep,
        false,
    ),
    rule(
        "đường",
        &["Đ.", "đướng", "đương", "duong", "dường", "đuờng"],
        AbbreviateAction::Replace("Đ."),
        RemoveAction::Remove,
        false,
    ),
    rule(
        "phố",
        &["phó"],
        AbbreviateAction::Replace("P."),
        RemoveAction::Remove,
        false,
    ),
];
