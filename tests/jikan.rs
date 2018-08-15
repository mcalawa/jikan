extern crate chrono;
extern crate jikan;
use jikan::jpn::Period;
use jikan::jpn::{Era, Court, EraYear, Date};
use chrono::{NaiveDate};

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

#[test]
fn era_from_kanji_ignores_non_kanji_characters() {
    assert_eq!(Era::from_kanji("8aB!昭わ和.").unwrap(), Era::new("Shouwa", "昭和", "しょうわ", "Shōwa", "Syôwa", NaiveDate::from_ymd(1926, 12, 25), NaiveDate::from_ymd(1989, 1, 7), 64, Court::Unified, None, None));
}

#[test]
fn era_from_kanji_errs_on_invalid_era_kanji() {
    assert!(Era::from_kanji("生花").is_err());
}

#[test]
fn era_from_georgian_year_returns_correct_number_of_eras() {
    assert_eq!(Era::from_georgian_year(1375).unwrap().len(), 4);
    assert_eq!(Era::from_georgian_year(1334).unwrap().len(), 2);
    assert_eq!(Era::from_georgian_year(1350).unwrap().len(), 3);
    assert_eq!(Era::from_georgian_year(686).unwrap().len(), 1);
}

#[test]
fn era_from_georgian_year_returns_error_for_invalid_years() {
    assert!(Era::from_georgian_year(2020).is_err());
    assert!(Era::from_georgian_year(670).is_err());
    assert!(Era::from_georgian_year(10).is_err());
    assert!(Era::from_georgian_year(692).is_err());
}

#[test]
fn era_from_georgian_year_returns_correct_eras() {
    let seven_oh_four : Vec<Era> = Era::from_georgian_year(704).unwrap();
    let thirteen_seventy_five : Vec<Era> = Era::from_georgian_year(1375).unwrap();
    let twenty_nineteen : Vec<Era> = Era::from_georgian_year(2019).unwrap();
    let fourteen_seventy : Vec<Era> = Era::from_georgian_year(1470).unwrap();

    assert_eq!(seven_oh_four[0].kana_spelling(), "Taihou".to_string());
    assert_eq!(seven_oh_four[1].hepburn(), "Keiun".to_string());

    assert_eq!(thirteen_seventy_five[0].iso_3602(), "Ôan".to_string());
    assert_eq!(thirteen_seventy_five[1].hiragana(), "ぶんちゅう".to_string());
    assert_eq!(thirteen_seventy_five[2].hepburn(), "Eiwa".to_string());
    assert_eq!(thirteen_seventy_five[3].kanji(), "天授".to_string());

    assert_eq!(twenty_nineteen[0].ged(), NaiveDate::from_ymd(2019, 4, 30));

    assert_eq!(fourteen_seventy[0].gsd(), NaiveDate::from_ymd(1469, 6, 17));
}

#[test]
fn era_from_georgian_returns_error_for_invalid_dates() {
    assert!(Era::from_georgian(NaiveDate::from_ymd(620, 5, 10)).is_err());
    assert!(Era::from_georgian(NaiveDate::from_ymd(-10, 5, 10)).is_err());
    assert!(Era::from_georgian(NaiveDate::from_ymd(666, 5, 10)).is_err());
    assert!(Era::from_georgian(NaiveDate::from_ymd(699, 5, 10)).is_err());
    assert!(Era::from_georgian(NaiveDate::from_ymd(2025, 5, 10)).is_err());
}

#[test]
fn era_from_georgian_returns_correct_eras() {
    assert_eq!(Era::from_georgian(NaiveDate::from_ymd(650, 4, 15)).unwrap()[0].kana_spelling(), "Hakuchi".to_string());
    assert_eq!(Era::from_georgian(NaiveDate::from_ymd(1375, 7, 4)).unwrap().len(), 3);
}

#[test]
fn era_from_gymd_returns_correct_eras() {
    assert_eq!(Era::from_georgian(NaiveDate::from_ymd(650, 4, 15)), Era::from_gymd(650, 4, 15));
    assert_eq!(Era::from_georgian(NaiveDate::from_ymd(1375, 7, 4)), Era::from_gymd(1375, 7, 4));
}

#[test]
fn era_year_from_georgian_returns_error_for_invalid_dates() {
    assert!(EraYear::from_georgian(NaiveDate::from_ymd(620, 5, 10)).is_err());
    assert!(EraYear::from_georgian(NaiveDate::from_ymd(-10, 5, 10)).is_err());
    assert!(EraYear::from_georgian(NaiveDate::from_ymd(666, 5, 10)).is_err());
    assert!(EraYear::from_georgian(NaiveDate::from_ymd(699, 5, 10)).is_err());
    assert!(EraYear::from_georgian(NaiveDate::from_ymd(2025, 5, 10)).is_err());
}

#[test]
fn era_year_from_georgian_returns_correct_era_years() {
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(650, 3, 25)).unwrap().len(), 2);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(650, 3, 25)).unwrap()[0].year(), 6);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(650, 3, 25)).unwrap()[0].era().hepburn(), "Taika".to_string());
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(650, 3, 25)).unwrap()[1].year(), 1);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(650, 3, 25)).unwrap()[1].era().iso_3602(), "Hakuti".to_string());

    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(650, 1, 15)).unwrap().len(), 1);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(650, 1, 15)).unwrap()[0].year(), 5);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(650, 1, 15)).unwrap()[0].era().kana_spelling(), "Taika".to_string());

    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(974, 1, 23)).unwrap()[0].year(), 1);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(974, 3, 15)).unwrap()[0].year(), 2);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(975, 1, 15)).unwrap()[0].year(), 2);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(975, 2, 22)).unwrap()[0].year(), 3);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(976, 1, 15)).unwrap()[0].year(), 3);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(976, 4, 2)).unwrap()[0].year(), 4);

    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(1868, 10, 31)).unwrap()[0].year(), 1);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(1869, 1, 31)).unwrap()[0].year(), 1);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(1869, 5, 31)).unwrap()[0].year(), 2);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(1870, 1, 31)).unwrap()[0].year(), 2);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(1870, 10, 31)).unwrap()[0].year(), 3);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(1871, 1, 31)).unwrap()[0].year(), 3);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(1871, 12, 31)).unwrap()[0].year(), 4);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(1872, 1, 31)).unwrap()[0].year(), 4);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(1872, 10, 31)).unwrap()[0].year(), 5);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(1873, 1, 1)).unwrap()[0].year(), 6);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(1891, 1, 1)).unwrap()[0].year(), 24);

    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(2018, 8, 14)).unwrap()[0].year(), 30);

    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(1375, 7, 4)).unwrap()[0].era().hepburn(), "Bunchū".to_string());
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(1375, 7, 4)).unwrap()[0].year(), 4);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(1375, 7, 4)).unwrap()[1].era().hepburn(), "Eiwa".to_string());
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(1375, 7, 4)).unwrap()[1].year(), 1);
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(1375, 7, 4)).unwrap()[2].era().hepburn(), "Tenju".to_string());
    assert_eq!(EraYear::from_georgian(NaiveDate::from_ymd(1375, 7, 4)).unwrap()[2].year(), 1);
}

#[test]
fn era_year_to_georgian_year_returns_correct_years() {
    let meiji = Era::from_kanji("明治").unwrap();
    let heisei = Era::from_kanji("平成").unwrap();

    assert_eq!(EraYear::new(meiji.clone(), 3).unwrap().to_georgian_year()[0], 1870);
    assert_eq!(EraYear::new(meiji.clone(), 3).unwrap().to_georgian_year()[1], 1871);
    assert_eq!(EraYear::new(meiji.clone(), 3).unwrap().to_georgian_year().len(), 2);
    assert_eq!(EraYear::new(meiji.clone(), 24).unwrap().to_georgian_year()[0], 1891);
    assert_eq!(EraYear::new(meiji, 24).unwrap().to_georgian_year().len(), 1);

    assert_eq!(EraYear::new(heisei, 30).unwrap().to_georgian_year()[0], 2018);
}

#[test]
fn date_from_georgian_returns_errors_for_invalid_dates() {
    assert!(Date::from_georgian(NaiveDate::from_ymd(420, 4, 20)).is_err());
    assert!(Date::from_georgian(NaiveDate::from_ymd(666, 6, 6)).is_err());
    assert!(Date::from_georgian(NaiveDate::from_ymd(700, 4, 20)).is_err());
    assert!(Date::from_georgian(NaiveDate::from_ymd(2025, 5, 31)).is_err());
}

#[test] 
fn date_from_georgian_returns_correct_dates() {
    let bunchuu = Era::from_kanji("文中").unwrap();
    let bunchuu4 = EraYear::new(bunchuu, 4).unwrap();

    let eiwa = Era::from_kanji("永和").unwrap();
    let eiwa1 = EraYear::new(eiwa, 1).unwrap();

    let tenju = Era::from_kanji("天授").unwrap();
    let tenju1 = EraYear::new(tenju, 1).unwrap();

    assert_eq!(Date::new(bunchuu4, 7, 4), Date::from_georgian(NaiveDate::from_ymd(1375, 7, 4)).unwrap()[0]);
    assert_eq!(Date::new(eiwa1, 7, 4), Date::from_georgian(NaiveDate::from_ymd(1375, 7, 4)).unwrap()[1]);
    assert_eq!(Date::new(tenju1, 7, 4), Date::from_georgian(NaiveDate::from_ymd(1375, 7, 4)).unwrap()[2]);

    let meiji = Era::from_kanji("明治").unwrap();
    let meiji3 = EraYear::new(meiji.clone(), 3).unwrap();
    let meiji24 = EraYear::new(meiji, 24).unwrap();

    assert_eq!(Date::new(meiji3.clone(), 10, 31), Date::from_georgian(NaiveDate::from_ymd(1870, 10, 31)).unwrap()[0]);
    assert_eq!(Date::new(meiji3, 1, 31), Date::from_georgian(NaiveDate::from_ymd(1871, 1, 31)).unwrap()[0]);
    assert_eq!(Date::new(meiji24, 1, 1), Date::from_georgian(NaiveDate::from_ymd(1891, 1, 1)).unwrap()[0]);

    let heisei = Era::from_kanji("平成").unwrap();
    let heisei30 = EraYear::new(heisei, 30).unwrap();

    assert_eq!(Date::new(heisei30, 8, 14), Date::from_georgian(NaiveDate::from_ymd(2018, 8, 14)).unwrap()[0]);
}

#[test]
fn date_to_georgian_returns_input_of_date_from_georgian() {
    assert_eq!(Date::from_georgian(NaiveDate::from_ymd(1375, 7, 4)).unwrap()[0].to_georgian(), NaiveDate::from_ymd(1375, 7, 4));
    assert_eq!(Date::from_georgian(NaiveDate::from_ymd(1375, 7, 4)).unwrap()[1].to_georgian(), NaiveDate::from_ymd(1375, 7, 4));
    assert_eq!(Date::from_georgian(NaiveDate::from_ymd(1375, 7, 4)).unwrap()[2].to_georgian(), NaiveDate::from_ymd(1375, 7, 4));

    assert_eq!(Date::from_georgian(NaiveDate::from_ymd(1870, 10, 31)).unwrap()[0].to_georgian(), NaiveDate::from_ymd(1870, 10, 31));
    assert_eq!(Date::from_georgian(NaiveDate::from_ymd(1871, 1, 31)).unwrap()[0].to_georgian(), NaiveDate::from_ymd(1871, 1, 31));
    assert_eq!(Date::from_georgian(NaiveDate::from_ymd(1891, 1, 1)).unwrap()[0].to_georgian(), NaiveDate::from_ymd(1891, 1, 1));

    assert_eq!(Date::from_georgian(NaiveDate::from_ymd(2018, 8, 14)).unwrap()[0].to_georgian(), NaiveDate::from_ymd(2018, 8, 14));
}