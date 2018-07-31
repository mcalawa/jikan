extern crate jikan;
use jikan::jpn::*;

#[test]
fn period_name_read_as_lowercase() {
    assert_eq!(Period::from_str("Asuka"), Some(&Period::new("Asuka jidai", "飛鳥時代", "あすかじだい", "Asuka jidai", "Asuka zidai", "Asuka zidai", "Asuka period", 538, 710)));
}