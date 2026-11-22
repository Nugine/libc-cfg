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
