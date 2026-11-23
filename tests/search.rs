use libc_cfg::RegexSet;
use libc_cfg::search;

/// `s2!` and `s_no_extra_traits2!` are libc's newer item-group macros.
/// Items behind them must be discovered like items in `s!`.
#[test]
fn finds_items_in_s2_macros() {
    let re = RegexSet::new(["^rlimit$", "^rlimit64$"]).unwrap();
    let libc = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/libc");

    let mut items = search(libc, &re).unwrap();
    items.sort_by(|lhs, rhs| lhs.name.cmp(&rhs.name));

    let names: Vec<&str> = items.iter().map(|item| item.name.as_str()).collect();
    assert_eq!(names, ["rlimit", "rlimit64"]);

    for item in &items {
        let cfg = item.cfg.to_string();
        assert!(
            cfg.contains("unix"),
            "unexpected cfg for {}: {cfg}",
            item.name
        );
    }
}
/// Definitions marked `#[deprecated]` are excluded from `cfg` and reported
/// through `deprecated_cfg`.
#[test]
fn separates_deprecated_definitions() {
    let re = RegexSet::new(["^MIXED$", "^DEPRECATED_ONLY$"]).unwrap();
    let libc = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/libc");

    let items = search(libc, &re).unwrap();
    let find = |name: &str| items.iter().find(|item| item.name == name).unwrap();

    let mixed = find("MIXED");
    let cfg = mixed.cfg.to_string();
    let deprecated = mixed.deprecated_cfg.to_string();
    assert!(cfg.contains("unix"), "unexpected cfg for MIXED: {cfg}");
    assert!(
        !cfg.contains("fuchsia"),
        "deprecated cfg leaked into MIXED: {cfg}"
    );
    assert!(
        deprecated.contains("fuchsia"),
        "unexpected deprecated_cfg for MIXED: {deprecated}"
    );

    let only_deprecated = find("DEPRECATED_ONLY");
    assert!(
        only_deprecated.cfg.is_const_false(),
        "deprecated-only item must not be available: {}",
        only_deprecated.cfg
    );
    assert!(
        only_deprecated
            .deprecated_cfg
            .to_string()
            .contains("fuchsia")
    );
}
