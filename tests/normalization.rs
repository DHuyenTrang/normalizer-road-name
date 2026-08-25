use proptest::prelude::*;
use road_name_normalizer::{normalize, Mode};

#[test]
fn approved_fixture_matches() {
    let fixture = include_str!("fixtures/normalization.csv");
    for (line_number, line) in fixture.lines().enumerate().skip(1) {
        let columns: Vec<_> = line.splitn(5, ',').collect();
        assert_eq!(columns.len(), 5, "invalid fixture row {}", line_number + 1);
        let mode = match columns[1] {
            "abbreviate" => Mode::Abbreviate,
            "remove" => Mode::Remove,
            value => panic!("unknown mode {value:?} on row {}", line_number + 1),
        };
        assert_eq!(
            normalize(columns[0], mode),
            columns[2],
            "fixture row {} ({})",
            line_number + 1,
            columns[3]
        );
    }
}

#[test]
fn matching_is_case_insensitive_and_nfc_aware() {
    assert_eq!(
        normalize("đƯỜnG   Nguyễn Trãi", Mode::Abbreviate),
        "Đ. Nguyễn Trãi"
    );
    assert_eq!(
        normalize("Đươ\u{0300}ng Nguyễn Trãi", Mode::Abbreviate),
        "Đ. Nguyễn Trãi"
    );
}

#[test]
fn unmatched_input_only_changes_its_first_letter() {
    assert_eq!(normalize("  nguyễn  trãi", Mode::Remove), "  Nguyễn  trãi");
    assert_eq!(
        normalize("1/2 Đường Nguyễn Trãi", Mode::Remove),
        "1/2 Đường Nguyễn Trãi"
    );
    assert_eq!(normalize("--", Mode::Abbreviate), "--");
}

#[test]
fn matching_requires_a_valid_boundary_and_only_handles_the_first_prefix() {
    assert_eq!(normalize("Đườngsắt", Mode::Remove), "Đườngsắt");
    for code_like_word in ["QLong", "ĐTx", "TLx", "HLx", "VĐx"] {
        assert_eq!(normalize(code_like_word, Mode::Abbreviate), code_like_word);
    }
    assert_eq!(normalize("Đường Phố Huế", Mode::Remove), "Phố Huế");
    assert_eq!(
        normalize("Nhánh Đường Nguyễn Trãi", Mode::Remove),
        "Nhánh Đường Nguyễn Trãi"
    );
}

#[test]
fn changed_boundary_whitespace_is_collapsed_but_remainder_is_preserved() {
    assert_eq!(
        normalize("đường   Nguyễn  Trãi ", Mode::Abbreviate),
        "Đ. Nguyễn  Trãi "
    );
    assert_eq!(normalize("Đường", Mode::Remove), "");
}

#[test]
fn every_approved_alias_is_canonicalized_in_both_modes() {
    let cases = [
        ("ĐT 2", "ĐT. 2", "ĐT. 2"),
        ("ĐT.2", "ĐT.2", "ĐT.2"),
        ("DT2", "ĐT.2", "ĐT.2"),
        ("QL 2", "QL. 2", "QL. 2"),
        ("QL.2", "QL.2", "QL.2"),
        ("TL 2", "TL. 2", "TL. 2"),
        ("TL.2", "TL.2", "TL.2"),
        ("HL 2", "HL. 2", "HL. 2"),
        ("HL.2", "HL.2", "HL.2"),
        ("VĐ 2", "VĐ. 2", "2"),
        ("VĐ.2", "VĐ.2", "2"),
        ("Hèm 2", "Hẻm 2", "Hẻm 2"),
        ("Hem 2", "Hẻm 2", "Hẻm 2"),
        ("Hẽm 2", "Hẻm 2", "Hẻm 2"),
        ("Đ. Nguyễn Trãi", "Đ. Nguyễn Trãi", "Nguyễn Trãi"),
        ("Đướng Nguyễn Trãi", "Đ. Nguyễn Trãi", "Nguyễn Trãi"),
        ("Đương Nguyễn Trãi", "Đ. Nguyễn Trãi", "Nguyễn Trãi"),
        ("Duong Nguyễn Trãi", "Đ. Nguyễn Trãi", "Nguyễn Trãi"),
        ("Dường Nguyễn Trãi", "Đ. Nguyễn Trãi", "Nguyễn Trãi"),
        ("Đuờng Nguyễn Trãi", "Đ. Nguyễn Trãi", "Nguyễn Trãi"),
        ("Phó Nguyễn Du", "P. Nguyễn Du", "Nguyễn Du"),
    ];

    for (input, abbreviated, removed) in cases {
        assert_eq!(normalize(input, Mode::Abbreviate), abbreviated, "{input}");
        assert_eq!(normalize(input, Mode::Remove), removed, "{input}");
    }
}

#[test]
fn every_word_form_is_ignored_outside_the_start_position() {
    let forms = [
        "Đường cao tốc",
        "Đường vành đai",
        "Đường liên thôn",
        "Đường liên xã",
        "Đường tỉnh",
        "Đường huyện",
        "Quốc lộ",
        "Tỉnh lộ",
        "Huyện lộ",
        "Đại lộ",
        "Cao tốc",
        "Xa lộ",
        "Vành đai",
        "Hẻm",
        "Ngách",
        "Ngõ",
        "Kiệt",
        "Đường",
        "Phố",
        "QLong",
        "ĐTx",
        "TLx",
        "HLx",
        "VĐx",
    ];

    for form in forms {
        let input = format!("Nhánh {form}");
        assert_eq!(normalize(&input, Mode::Remove), input, "{form}");
    }
}

proptest! {
    #[test]
    fn arbitrary_utf8_never_panics(input in ".{0,256}") {
        for mode in [Mode::Abbreviate, Mode::Remove] {
            let _ = normalize(&input, mode);
        }
    }

    #[test]
    fn abbreviation_is_idempotent(input in ".{0,256}") {
        let once = normalize(&input, Mode::Abbreviate);
        prop_assert_eq!(normalize(&once, Mode::Abbreviate), once);
    }
}
