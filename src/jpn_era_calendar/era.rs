// This is part of Jikan
// See README.md for details

use chrono::prelude::*;
use period::Period;

#[derive(Clone)]
pub struct Era {
    kana_spelling: &str,
    kanji: &str,
    hiragana: &str,
    hepburn: &str,
    iso_3602: &str,
    start_georgian: NaiveDate,
    end_georgian: NaiveDate,
    end_year: u32,
}

impl Era {
    fn new(kana_spelling: &str, kanji: &str, hiragana: &str, hepburn: &str, iso_3602: &str, start_georgian: NaiveDate, end_georgian: NaiveDate, end_year: u32) -> Self {
        let era = Era {
            self::kana_spelling = kana_spelling,
            self::kanji = kanji,
            self::hiragana = hiragana,
            self::hepburn = hepburn,
            self::iso_3602 = iso_3602,
            self::start_georgian = start_georgian,
            self::end_georgian = end_georgian,
            self::end_year = end_year
        };
        era
    }

    pub fn from_kanji(kanji: &str) -> Option<Self> {
        if kanji == "大化" {
            Some(Era::new("Taika", "大化", "たいか", "Taika", "Taika", NaiveDate::from_ymd(645, 7, 20), NaiveDate::from_ymd(650, 3, 25), 6))
        }
        else if kanji == "白雉" {
            Some(Era::new("Hakuchi", "白雉", "はくち", "Hakuchi", "Hakuti", NaiveDate::from_ymd(650, 3, 25), NaiveDate::from_ymd(654, 11, 27), 5))
        }
        else if kanji == "朱鳥" {
            Some(Era::new("Shuchou", "朱鳥", "しゅちょう", "Shuchō", "Syutyô", NaiveDate::from_ymd(686, 8, 17), NaiveDate::from_ymd(686, 10, 4), 1))
        }
        else if kanji == "大宝" {
            Some(Era::new("Taihou", "大宝", "たいほう", "Taihō", "Taihô", NaiveDate::from_ymd(701, 5, 7), NaiveDate::from_ymd(704, 6, 20), 4))
        }
        else if kanji == "慶雲" {
            Some(Era::new("Keiun", "慶雲", "けいうん", "Keiun", "Keiun", NaiveDate::from_ymd(704, 6, 20), NaiveDate::from_ymd(708, 2, 11), 5))
        }
        else if kanji == "和銅" {
            Some(Era::new("Wadou", "和銅", "わどう", "Wadō", "Wadô", NaiveDate::from_ymd(708, 2, 11), NaiveDate::from_ymd(715, 10, 7), 8))
        }
        else {
            None
        }
    }

    pub fn from_kana_spelling(era: &str, period: &str) -> Option<Self> {
        if era.to_lowercase() == "taika" {
            Era::from_kanji("大化")
        }
        else if era.to_lowercase()
        unimplemented!()
    }

    pub fn from_hepburn(era: &str, period: &str) -> Option<Self> {
        unimplemented!()
    }

    pub fn from_iso_3602(era: &str, period: &str) -> Option<Self> {
        unimplemented!()
    }

    pub fn from_name_and_period(era: &str, period: Period) -> Option<Self> {
        unimplemented!()
    }
}