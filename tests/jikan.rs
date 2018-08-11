extern crate jikan;
use jikan::jpn::*;

#[test]
fn period_name_read_as_lowercase() {
    assert_eq!(*Period::from_str("Asuka").unwrap(), Period::new("Asuka jidai", "飛鳥時代", "あすかじだい", "Asuka jidai", "Asuka zidai", "Asuka zidai", "Asuka period", 538, 710));
}

#[test]
fn period_name_after_hyphen_not_read() {
    assert_eq!(*Period::from_str("nanboku-chou").unwrap(), Period::new("Nanboku-chou jidai", "南北朝時代", "なんぼくちょうじだい", "Nanboku-chō jidai", "Nanboku-tyô zidai", "Nanboku-tyô zidai", "Northern and Southern Courts period", 1336, 1392));
}

#[test]
fn period_name_ignores_punctuation_other_than_hyphen() {
    assert_eq!(*Period::from_str("n.an&b$ok^u#ch@ou").unwrap(), Period::new("Nanboku-chou jidai", "南北朝時代", "なんぼくちょうじだい", "Nanboku-chō jidai", "Nanboku-tyô zidai", "Nanboku-tyô zidai", "Northern and Southern Courts period", 1336, 1392));
}

#[test]
fn period_name_ignores_leading_whitespace() {
    assert_eq!(*Period::from_str("   
    asuka").unwrap(), Period::new("Asuka jidai", "飛鳥時代", "あすかじだい", "Asuka jidai", "Asuka zidai", "Asuka zidai", "Asuka period", 538, 710));
}

#[test]
fn period_name_ignores_numeric_characters() {
    assert_eq!(*Period::from_str("m7u3r977omachi").unwrap(), Period::new("Muromachi jidai", "室町時代", "むろまちじだい", "Muromachi jidai", "Muromati zidai", "Muromati zidai", "Muromachi period", 1336, 1573));
}

#[test]
fn period_name_ignores_words_after_first_nonleading_whitespace() {
    assert_eq!(*Period::from_str("Kenmu restoration").unwrap(), Period::new("Kenmu no shinsei", "建武の新政", "けんむのしんせい", "Kenmu no shinsei", "Kenmu no sinsei", "Kenmu no sinsei", "Kenmu restoration", 1333, 1336));
}