use chrono::prelude::*;
pub use era::Era;

pub struct Period {
    kana_spelling: &str,
    kanji: &str,
    hiragana: &str,
    hepburn: &str,
    iso_3602: &str,
    iso_3602_strict: &str,
    translation: &str,
    georgian_start_year: u32,
    georgian_end_year: u32,
}

impl Period {
    fn new(kana_spelling: &str, kanji: &str, hiragana: &str, hepburn: &str, iso_3602: &str, iso_3602_strict: &str, translation: &str, georgian_start_year: u32, georgian_end_year: u32) -> Self {
        let period = Period {
            self::kana_spelling = kana_spelling,
            self::kanji = kanji,
            self::hiragana = hiragana,
            self::hepburn = hepburn,
            self::iso_3602 = iso_3602,
            self::iso_3602_strict = iso_3602_strict,
            self::translation = translation,
            self::georgian_start_year = georgian_start_year,
            self::georgian_end_year = georgian_end_year
        };
        period
    }
    
    pub fn from_kanji(kanji: &str) -> Option<Self> {
        if kanji == "飛鳥時代" {
            Some(Period::new("Asuka jidai", "飛鳥時代", "あすかじだい", "Asuka jidai", "Asuka zidai", "Asuka zidai", "Asuka period", 538, 710))
        }
        else if kanji == "奈良時代" {
            Some(Period::new("Nara jidai", "奈良時代", "ならじだい", "Nara jidai", "Nara zidai", "Nara zidai", "Nara period", 710, 794))
        }
        else if kanji == "平安時代" {
            Some(Period::new("Heian jidai", "平安時代", "へいあんじだい", "Heian jidai", "Heian zidai", "Heian zidai", "Heian period", 794, 1185))
        }
        else if kanji == "鎌倉時代" {
            Some(Period::new("Kamakura jidai", "鎌倉時代", "かまくらじだい", "Kamakura jidai", "Kamakura zidai", "Kamakura zidai", "Kamakura period", 1185, 1333))
        }
        else if kanji == "建武の新政" {
            Some(Period::new("Kenmu no shinsei", "建武の新政", "けんむのしんせい", "Kenmu no shinsei", "Kenmu no sinsei", "Kenmu no sinsei", "Kenmu restoration", 1333, 1336))
        }
        else if kanji == "室町時代" {
            Some(Period::new("Muromachi jidai", "室町時代", "むろまちじだい", "Muromachi jidai", "Muromati zidai", "Muromati zidai", "Muromachi period", 1336, 1573))
        }
        else if kanji == "足利時代" {
            Some(Period::new("Ashikaga jidai", "足利時代", "あしかがじだい", "Ashikaga jidai", "Asikaga zidai", "Asikaga zidai", "Ashikaga period", 1336, 1573))
        }
        else if kanji == "南北朝時代" {
            Some(Period::new("Nanboku-chou jidai", "南北朝時代", "なんぼくちょうじだい", "Nanboku-chō jidai", "Nanboku-tyô zidai", "Nanboku-tyô zidai", "Northern and Southern Courts period", 1336, 1392))
        }
        else if kanji == "戦国時代" {
            Some(Period::new("Sengoku jidai", "戦国時代", "せんごくじだい", "Sengoku jidai", "Sengoku zidai", "Sengoku zidai", "Warring States period", 1467, 1568))
        }
        else if kanji == "安土桃山時代" {
            Some(Period::new("Aduchi-Momoyama jidai", "安土桃山時代", "あづちももやまじだい", "Azuchi-Momoyama jidai", "Azuti-Momoyama zidai", "Aduti-Momoyama zidai", "Azuchi-Momoyama period", 1573, 1603))
        }
        else if kanji == "江戸時代" {
            Some(Period::new("Edo jidai", "江戸時代", "えどじだい", "Edo jidai", "Edo zidai", "Edo zidai", "Edo period", 1603, 1868))
        }
        else if kanji == "徳川時代" {
            Some(Period::new("Tokugawa jidai", "徳川時代", "とくがわじだい", "Tokugawa jidai", "Tokugawa zidai", "Tokugawa zidai", "Tokugawa period", 1603, 1868))
        }
        else if kanji == "明治時代" {
            Some(Period::new("Meiji jidai", "明治時代", "めいじじだい", "Meiji jidai", "Meizi zidai", "Meizi zidai", "Meiji period", 1868, 1912))
        }
        else if kanji == "大正時代" {
            Some(Period::new("Taishou jidai", "大正時代", "たいしょうじだい", "Taishō jidai", "Taisyô zidai", "Taisyô zidai", "Taisho period", 1912, 1926))
        }
        else if kanji == "昭和時代" {
            Some(Period::new("Shouwa jidai", "昭和時代", "しょうわじだい", "Shōwa jidai", "Syôwa zidai", "Syôwa zidai", "Showa period", 1926, 1989))
        }
        else if kanji == "平成時代" {
            Some(Period::new("Heisei jidai", "平成時代", "へいせいじだい", "Heisei jidai", "Heisei zidai", "Heisei zidai", "Heisei period", 1989, 2019))
        }
        else {
            None
        }
    }

    pub fn from_kana_spelling(kana_spelling: &str) -> Option<Self> {
        if kana_spelling.to_lowercase() == "asuka jidai" {
            Period::from_kanji("飛鳥時代")
        }
        else if kana_spelling.to_lowercase() == "asukajidai" {
            Period::from_kanji("飛鳥時代")
        }
        else if kana_spelling.to_lowercase() == "asuka" {
            Period::from_kanji("飛鳥時代")
        }
        else if kana_spelling.to_lowercase() == "nara jidai" {
            Period::from_kanji("奈良時代")
        }
        else if kana_spelling.to_lowercase() == "narajidai" {
            Period::from_kanji("奈良時代")
        }
        else if kana_spelling.to_lowercase() == "nara" {
            Period::from_kanji("奈良時代")
        }
        else if kana_spelling.to_lowercase() == "heian jidai" {
            Period::from_kanji("平安時代")
        }
        else if kana_spelling.to_lowercase() == "heianjidai" {
            Period::from_kanji("平安時代")
        }
        else if kana_spelling.to_lowercase() == "heian" {
            Period::from_kanji("平安時代")
        }
        else if kana_spelling.to_lowercase() == "kamakura jidai" {
            Period::from_kanji("鎌倉時代")
        }
        else if kana_spelling.to_lowercase() == "kamakurajidai" {
            Period::from_kanji("鎌倉時代")
        }
        else if kana_spelling.to_lowercase() == "kamakura" {
            Period::from_kanji("鎌倉時代")
        }
        else if kana_spelling.to_lowercase() == "kenmu no shinsei" {
            Period::from_kanji("建武の新政")
        }
        else if kana_spelling.to_lowercase() == "kenmunoshinsei" {
            Period::from_kanji("建武の新政")
        }
        else if kana_spelling.to_lowercase() == "kenmu" {
            Period::from_kanji("建武の新政")
        }
        else if kana_spelling.to_lowercase() == "muromachi jidai" {
            Period::from_kanji("室町時代")
        }
        else if kana_spelling.to_lowercase() == "muromachijidai" {
            Period::from_kanji("室町時代")
        }
        else if kana_spelling.to_lowercase() == "muromachi" {
            Period::from_kanji("室町時代")
        }
        else if kana_spelling.to_lowercase() == "ashikaga jidai" {
            Some(Period::from_kanji("足利時代"))
        }
        else if kana_spelling.to_lowercase() == "ashikagajidai" {
            Period::from_kanji("足利時代")
        }
        else if kana_spelling.to_lowercase() == "ashikaga" {
            Period::from_kanji("足利時代")
        }
        else if kana_spelling.to_lowercase() == "nanboku-chou jidai" {
            Period::from_kanji("南北朝時代")
        }
        else if kana_spelling.to_lowercase() == "nanbokuchoujidai" {
            Period::from_kanji("南北朝時代")
        }
        else if kana_spelling.to_lowercase() == "nanboku-chou" {
            Period::from_kanji("南北朝時代")
        }
        else if kana_spelling.to_lowercase() == "nanbokuchou" {
            Period::from_kanji("南北朝時代")
        }
        else if kana_spelling.to_lowercase() == "sengoku jidai" {
            Period::from_kanji("戦国時代")
        }
        else if kana_spelling.to_lowercase() == "sengokujidai" {
            Period::from_kanji("戦国時代")
        }
        else if kana_spelling.to_lowercase() == "sengoku" {
            Period::from_kanji("戦国時代")
        }
        else if kana_spelling.to_lowercase() == "aduchi-momoyama jidai" {
            Period::from_kanji("安土桃山時代")
        }
        else if kana_spelling.to_lowercase() == "aduchimomoyamajidai" {
            Period::from_kanji("安土桃山時代")
        }
        else if kana_spelling.to_lowercase() == "aduchi-Momoyama" {
            Period::from_kanji("安土桃山時代")
        }
        else if kana_spelling.to_lowercase() == "aduchimomoyama" {
            Period::from_kanji("安土桃山時代")
        }
        else if kana_spelling.to_lowercase() == "edo jidai" {
            Period::from_kanji("江戸時代")
        }
        else if kana_spelling.to_lowercase() == "edojidai" {
            Period::from_kanji("江戸時代")
        }
        else if kana_spelling.to_lowercase() == "edo" {
            Period::from_kanji("江戸時代")
        }
        else if kana_spelling.to_lowercase() == "tokugawa jidai" {
            Period::from_kanji("徳川時代")
        }
        else if kana_spelling.to_lowercase() == "tokugawajidai" {
            Period::from_kanji("徳川時代")
        }
        else if kana_spelling.to_lowercase() == "tokugawa" {
            Period::from_kanji("徳川時代")
        }
        else if kana_spelling.to_lowercase() == "meiji jidai" {
            Period::from_kanji("明治時代")
        }
        else if kana_spelling.to_lowercase() == "meijijidai" {
            Period::from_kanji("明治時代")
        }
        else if kana_spelling.to_lowercase() == "meiji" {
            Period::from_kanji("明治時代")
        }
        else if kana_spelling.to_lowercase() == "taishou jidai" {
            Period::from_kanji("大正時代")
        }
        else if kana_spelling.to_lowercase() == "taishoujidai" {
            Period::from_kanji("大正時代")
        }
        else if kana_spelling.to_lowercase() == "taishou" {
            Period::from_kanji("大正時代")
        }
        else if kana_spelling.to_lowercase() == "shouwa jidai" {
            Period::from_kanji("昭和時代")
        }
        else if kana_spelling.to_lowercase() == "shouwajidai" {
            Period::from_kanji("昭和時代")
        }
        else if kana_spelling.to_lowercase() == "shouwa" {
            Period::from_kanji("昭和時代")
        }
        else if kana_spelling.to_lowercase() == "heisei jidai" {
            Period::from_kanji("平成時代")
        }
        else if kana_spelling.to_lowercase() == "heiseijidai" {
            Period::from_kanji("平成時代")
        }
        else if kana_spelling.to_lowercase() == "heisei" {
            Period::from_kanji("平成時代")
        }
        else {
            None
        }
    }

    pub fn from_hiragana(hiragana: &str) -> Option<Self> {
        if hiragana == "あすかじだい" {
            Period::from_kanji("飛鳥時代")
        }
        else if hiragana == "あすか" {
            Period::from_kanji("飛鳥時代")
        }
        else if hiragana == "ならじだい" {
            Period::from_kanji("奈良時代")
        }
        else if hiragana == "なら" {
            Period::from_kanji("奈良時代")
        }
        else if hiragana == "へいあんじだい" {
            Period::from_kanji("平安時代")
        }
        else if hiragana == "へいあん" {
            Period::from_kanji("平安時代")
        }
        else if hiragana == "かまくらじだい" {
            Period::from_kanji("鎌倉時代")
        }
        else if hiragana == "かまくら" {
            Period::from_kanji("鎌倉時代")
        }
        else if hiragana == "けんむのしんせい" {
            Period::from_kanji("建武の新政")
        }
        else if hiragana == "けんむ" {
            Period::from_kanji("建武の新政")
        }
        else if hiragana == "むろまちじだい" {
            Period::from_kanji("室町時代")
        }
        else if hiragana == "むろまち" {
            Period::from_kanji("室町時代")
        }
        else if hiragana == "あしかがじだい" {
            Period::from_kanji("足利時代")
        }
        else if hiragana == "あしかが" {
            Period::from_kanji("足利時代")
        }
        else if hiragana == "なんぼくちょうじだい" {
            Period::from_kanji("南北朝時代")
        }
        else if hiragana == "せんごくじだい" {
            Period::from_kanji("戦国時代")
        }
        else if hiragana == "せんごく" {
            Period::from_kanji("戦国時代")
        }
        else if hiragana == "あづちももやまじだい" {
            Period::from_kanji("安土桃山時代")
        }
        else if hiragana == "あづちももやま" {
            Period::from_kanji("安土桃山時代")
        }
        else if hiragana == "えどじだい" {
            Period::from_kanji("江戸時代")
        }
        else if hiragana == "えど" {
            Period::from_kanji("江戸時代")
        }
        else if hiragana == "とくがわじだい" {
            Period::from_kanji("徳川時代")
        }
        else if hiragana == "とくがわ" {
            Period::from_kanji("徳川時代")
        }
        else if hiragana == "めいじじだい" {
            Period::from_kanji("明治時代")
        }
        else if hiragana == "めいじ" {
            Period::from_kanji("明治時代")
        }
        else if hiragana == "たいしょうじだい" {
            Period::from_kanji("大正時代")
        }
        else if hiragana == "たいしょう" {
            Period::from_kanji("大正時代")
        }
        else if hiragana == "しょうわじだい" {
            Period::from_kanji("昭和時代")
        }
        else if hiragana == "しょうわ" {
            Period::from_kanji("昭和時代")
        }
        else if hiragana == "へいせいじだい" {
            Period::from_kanji("平成時代")
        }
        else if hiragana == "へいせい" {
            Period::from_kanji("平成時代")
        }
        else {
            None
        }
    }

    pub fn from_hepburn(hepburn: &str) -> Option<Self> {
        if hepburn.to_lowercase() == "asuka jidai" {
            Period::from_kanji("飛鳥時代")
        }
        else if hepburn.to_lowercase() == "asukajidai" {
            Period::from_kanji("飛鳥時代")
        }
        else if hepburn.to_lowercase() == "asuka" {
            Period::from_kanji("飛鳥時代")
        }
        else if hepburn.to_lowercase() == "nara jidai" {
            Period::from_kanji("奈良時代")
        }
        else if hepburn.to_lowercase() == "narajidai" {
            Period::from_kanji("奈良時代")
        }
        else if hepburn.to_lowercase() == "nara" {
            Period::from_kanji("奈良時代")
        }
        else if hepburn.to_lowercase() == "heian jidai" {
            Period::from_kanji("平安時代")
        }
        else if hepburn.to_lowercase() == "heianjidai" {
            Period::from_kanji("平安時代")
        }
        else if hepburn.to_lowercase() == "heian" {
            Period::from_kanji("平安時代")
        }
        else if hepburn.to_lowercase() == "kamakura jidai" {
            Period::from_kanji("鎌倉時代")
        }
        else if hepburn.to_lowercase() == "kamakurajidai" {
            Period::from_kanji("鎌倉時代")
        }
        else if hepburn.to_lowercase() == "kamakura" {
            Period::from_kanji("鎌倉時代")
        }
        else if hepburn.to_lowercase() == "kenmu no shinsei" {
            Period::from_kanji("建武の新政")
        }
        else if hepburn.to_lowercase() == "kenmunoshinsei" {
            Period::from_kanji("建武の新政")
        }
        else if hepburn.to_lowercase() == "kenmu" {
            Period::from_kanji("建武の新政")
        }
        else if hepburn.to_lowercase() == "muromachi jidai" {
            Period::from_kanji("室町時代")
        }
        else if hepburn.to_lowercase() == "muromachijidai" {
            Period::from_kanji("室町時代")
        }
        else if hepburn.to_lowercase() == "muromachi" {
            Period::from_kanji("室町時代")
        }
        else if hepburn.to_lowercase() == "ashikaga jidai" {
            Period::from_kanji("足利時代")
        }
        else if hepburn.to_lowercase() == "ashikagajidai" {
            Period::from_kanji("足利時代")
        }
        else if hepburn.to_lowercase() == "ashikaga" {
            Period::from_kanji("足利時代")
        }
        else if hepburn.to_lowercase() == "nanboku-chō jidai" {
            Period::from_kanji("南北朝時代")
        }
        else if hepburn.to_lowercase() == "nanbokuchō jidai" {
            Period::from_kanji("南北朝時代")
        }
        else if hepburn.to_lowercase() == "nanbokuchōjidai" {
            Period::from_kanji("南北朝時代")
        }
        else if hepburn.to_lowercase() == "nanboku-chō" {
            Period::from_kanji("南北朝時代")
        }
        else if hepburn.to_lowercase() == "nanbokuchō" {
            Period::from_kanji("南北朝時代")
        }
        else if hepburn.to_lowercase() == "sengoku jidai" {
            Period::from_kanji("戦国時代")
        }
        else if hepburn.to_lowercase() == "sengokujidai" {
            Period::from_kanji("戦国時代")
        }
        else if hepburn.to_lowercase() == "sengoku" {
            Period::from_kanji("戦国時代")
        }
        else if hepburn.to_lowercase() == "azuchi-momoyama jidai" {
            Period::from_kanji("安土桃山時代")
        }
        else if hepburn.to_lowercase() == "azuchimomoyama jidai" {
            Period::from_kanji("安土桃山時代")
        }
        else if hepburn.to_lowercase() == "azuchimomoyamajidai" {
            Period::from_kanji("安土桃山時代")
        }
        else if hepburn.to_lowercase() == "azuchi-momoyama" {
            Period::from_kanji("安土桃山時代")
        }
        else if hepburn.to_lowercase() == "azuchimomoyama" {
            Period::from_kanji("安土桃山時代")
        }
        else if hepburn.to_lowercase() == "edo jidai" {
            Period::from_kanji("江戸時代")
        }
        else if hepburn.to_lowercase() == "edojidai" {
            Period::from_kanji("江戸時代")
        }
        else if hepburn.to_lowercase() == "edo" {
            Period::from_kanji("江戸時代")
        }
        else if hepburn.to_lowercase() == "tokugawa jidai" {
            Period::from_kanji("徳川時代")
        }
        else if hepburn.to_lowercase() == "tokugawajidai" {
            Period::from_kanji("徳川時代")
        }
        else if hepburn.to_lowercase() == "tokugawa" {
            Period::from_kanji("徳川時代")
        }
        else if hepburn.to_lowercase() == "meiji jidai" {
            Period::from_kanji("明治時代")
        }
        else if hepburn.to_lowercase() == "meijijidai" {
            Period::from_kanji("明治時代")
        }
        else if hepburn.to_lowercase() == "meiji" {
            Period::from_kanji("明治時代")
        }
        else if hepburn.to_lowercase() == "taishō jidai" {
            Period::from_kanji("大正時代")
        }
        else if hepburn.to_lowercase() == "taishōjidai" {
            Period::from_kanji("大正時代")
        }
        else if hepburn.to_lowercase() == "taishō" {
            Period::from_kanji("大正時代")
        }
        else if hepburn.to_lowercase() == "shōwa jidai" {
            Period::from_kanji("昭和時代")
        }
        else if hepburn.to_lowercase() == "shōwajidai" {
            Period::from_kanji("昭和時代")
        }
        else if hepburn.to_lowercase() == "shōwa" {
            Period::from_kanji("昭和時代")
        }
        else if hepburn.to_lowercase() == "heisei jidai" {
            Period::from_kanji("平成時代")
        }
        else if hepburn.to_lowercase() == "heiseijidai" {
            Period::from_kanji("平成時代")
        }
        else if hepburn.to_lowercase() == "heisei" {
            Period::from_kanji("平成時代")
        }
        else {
            None
        }
    }

    pub fn from_iso_3602(iso_3602: &str) -> Option<Self> {
        if iso_3602.to_lowercase() == "asuka zidai" {
            Period::from_kanji("飛鳥時代")
        }
        else if iso_3602.to_lowercase() == "asukazidai" {
            Period::from_kanji("飛鳥時代")
        }
        else if iso_3602.to_lowercase() == "asuka" {
            Period::from_kanji("飛鳥時代")
        }
        else if iso_3602.to_lowercase() == "nara zidai" {
            Period::from_kanji("奈良時代")
        }
        else if iso_3602.to_lowercase() == "narazidai" {
            Period::from_kanji("奈良時代")
        }
        else if iso_3602.to_lowercase() == "nara" {
            Period::from_kanji("奈良時代")
        }
        else if iso_3602.to_lowercase() == "heian zidai" {
            Period::from_kanji("平安時代")
        }
        else if iso_3602.to_lowercase() == "heianzidai" {
            Period::from_kanji("平安時代")
        }
        else if iso_3602.to_lowercase() == "heian" {
            Period::from_kanji("平安時代")
        }
        else if iso_3602.to_lowercase() == "kamakura zidai" {
            Period::from_kanji("鎌倉時代")
        }
        else if iso_3602.to_lowercase() == "kamakurazidai" {
            Period::from_kanji("鎌倉時代")
        }
        else if iso_3602.to_lowercase() == "kamakura" {
            Period::from_kanji("鎌倉時代")
        }
        else if iso_3602.to_lowercase() == "kenmu no sinsei" {
            Period::from_kanji("建武の新政")
        }
        else if iso_3602.to_lowercase() == "kenmunosinsei" {
            Period::from_kanji("建武の新政")
        }
        else if iso_3602.to_lowercase() == "kenmu" {
            Period::from_kanji("建武の新政")
        }
        else if iso_3602.to_lowercase() == "muromati zidai" {
            Period::from_kanji("室町時代")
        }
        else if iso_3602.to_lowercase() == "muromatizidai" {
            Period::from_kanji("室町時代")
        }
        else if iso_3602.to_lowercase() == "muromati" {
            Period::from_kanji("室町時代")
        }
        else if iso_3602.to_lowercase() == "asikaga zidai" {
            Period::from_kanji("足利時代")
        }
        else if iso_3602.to_lowercase() == "asikagazidai" {
            Period::from_kanji("足利時代")
        }
        else if iso_3602.to_lowercase() == "asikaga" {
            Period::from_kanji("足利時代")
        }
        else if iso_3602.to_lowercase() == "nanboku-tyô zidai" {
            Period::from_kanji("南北朝時代")
        }
        else if iso_3602.to_lowercase() == "nanbokutyô zidai" {
            Period::from_kanji("南北朝時代")
        }
        else if iso_3602.to_lowercase() == "nanbokutyôzidai" {
            Period::from_kanji("南北朝時代")
        }
        else if iso_3602.to_lowercase() == "nanboku-tyô" {
            Period::from_kanji("南北朝時代")
        }
        else if iso_3602.to_lowercase() == "nanbokutyô" {
            Period::from_kanji("南北朝時代")
        }
        else if iso_3602.to_lowercase() == "sengoku zidai" {
            Period::from_kanji("戦国時代")
        }
        else if iso_3602.to_lowercase() == "sengokuzidai" {
            Period::from_kanji("戦国時代")
        }
        else if iso_3602.to_lowercase() == "sengoku" {
            Period::from_kanji("戦国時代")
        }
        else if iso_3602.to_lowercase() == "azuti-momoyama zidai" {
            Period::from_kanji("安土桃山時代")
        }
        else if iso_3602.to_lowercase() == "azutimomoyama zidai" {
            Period::from_kanji("安土桃山時代")
        }
        else if iso_3602.to_lowercase() == "azutimomoyamazidai" {
            Period::from_kanji("安土桃山時代")
        }
        else if iso_3602.to_lowercase() == "azuti-momoyama" {
            Period::from_kanji("安土桃山時代")
        }
        else if iso_3602.to_lowercase() == "azutimomoyama" {
            Period::from_kanji("安土桃山時代")
        }
        else if iso_3602.to_lowercase() == "edo zidai" {
            Period::from_kanji("江戸時代")
        }
        else if iso_3602.to_lowercase() == "edozidai" {
            Period::from_kanji("江戸時代")
        }
        else if iso_3602.to_lowercase() == "edo" {
            Period::from_kanji("江戸時代")
        }
        else if iso_3602.to_lowercase() == "tokugawa zidai" {
            Period::from_kanji("徳川時代")
        }
        else if iso_3602.to_lowercase() == "tokugawazidai" {
            Period::from_kanji("徳川時代")
        }
        else if iso_3602.to_lowercase() == "tokugawa" {
            Period::from_kanji("徳川時代")
        }
        else if iso_3602.to_lowercase() == "meizi zidai" {
            Period::from_kanji("明治時代")
        }
        else if iso_3602.to_lowercase() == "meizizidai" {
            Period::from_kanji("明治時代")
        }
        else if iso_3602.to_lowercase() == "meizi" {
            Period::from_kanji("明治時代")
        }
        else if iso_3602.to_lowercase() == "taisyô zidai" {
            Period::from_kanji("大正時代")
        }
        else if iso_3602.to_lowercase() == "taisyôzidai" {
            Period::from_kanji("大正時代")
        }
        else if iso_3602.to_lowercase() == "taisyô" {
            Period::from_kanji("大正時代")
        }
        else if iso_3602.to_lowercase() == "syôwa zidai" {
            Period::from_kanji("昭和時代")
        }
        else if iso_3602.to_lowercase() == "syôwazidai" {
            Period::from_kanji("昭和時代")
        }
        else if iso_3602.to_lowercase() == "syôwa" {
            Period::from_kanji("昭和時代")
        }
        else if iso_3602.to_lowercase() == "heisei zidai" {
            Period::from_kanji("平成時代")
        }
        else if iso_3602.to_lowercase() == "heiseizidai" {
            Period::from_kanji("平成時代")
        }
        else if iso_3602.to_lowercase() == "heisei" {
            Period::from_kanji("平成時代")
        }
        else {
            None
        }
    }

    pub fn from_iso_3602_strict(iso_3602_strict: &str) -> Option<Self> {
        if iso_3602_strict.to_lowercase() == "asuka zidai" {
            Period::from_kanji("飛鳥時代")
        }
        else if iso_3602_strict.to_lowercase() == "asukazidai" {
            Period::from_kanji("飛鳥時代")
        }
        else if iso_3602_strict.to_lowercase() == "asuka" {
            Period::from_kanji("飛鳥時代")
        }
        else if iso_3602_strict.to_lowercase() == "nara zidai" {
            Period::from_kanji("奈良時代")
        }
        else if iso_3602_strict.to_lowercase() == "narazidai" {
            Period::from_kanji("奈良時代")
        }
        else if iso_3602_strict.to_lowercase() == "nara" {
            Period::from_kanji("奈良時代")
        }
        else if iso_3602_strict.to_lowercase() == "heian zidai" {
            Period::from_kanji("平安時代")
        }
        else if iso_3602_strict.to_lowercase() == "heianzidai" {
            Period::from_kanji("平安時代")
        }
        else if iso_3602_strict.to_lowercase() == "heian" {
            Period::from_kanji("平安時代")
        }
        else if iso_3602_strict.to_lowercase() == "kamakura zidai" {
            Period::from_kanji("鎌倉時代")
        }
        else if iso_3602_strict.to_lowercase() == "kamakurazidai" {
            Period::from_kanji("鎌倉時代")
        }
        else if iso_3602_strict.to_lowercase() == "kamakura" {
            Period::from_kanji("鎌倉時代")
        }
        else if iso_3602_strict.to_lowercase() == "kenmu no sinsei" {
            Period::from_kanji("建武の新政")
        }
        else if iso_3602_strict.to_lowercase() == "kenmunosinsei" {
            Period::from_kanji("建武の新政")
        }
        else if iso_3602_strict.to_lowercase() == "kenmu" {
            Period::from_kanji("建武の新政")
        }
        else if iso_3602_strict.to_lowercase() == "muromati zidai" {
            Period::from_kanji("室町時代")
        }
        else if iso_3602_strict.to_lowercase() == "muromatizidai" {
            Period::from_kanji("室町時代")
        }
        else if iso_3602_strict.to_lowercase() == "muromati" {
            Period::from_kanji("室町時代")
        }
        else if iso_3602_strict.to_lowercase() == "asikaga zidai" {
            Period::from_kanji("足利時代")
        }
        else if iso_3602_strict.to_lowercase() == "asikagazidai" {
            Period::from_kanji("足利時代")
        }
        else if iso_3602_strict.to_lowercase() == "asikaga" {
            Period::from_kanji("足利時代")
        }
        else if iso_3602_strict.to_lowercase() == "nanboku-tyô zidai" {
            Period::from_kanji("南北朝時代")
        }
        else if iso_3602_strict.to_lowercase() == "nanbokutyô zidai" {
            Period::from_kanji("南北朝時代")
        }
        else if iso_3602_strict.to_lowercase() == "nanbokutyôzidai" {
            Period::from_kanji("南北朝時代")
        }
        else if iso_3602_strict.to_lowercase() == "nanboku-tyô" {
            Period::from_kanji("南北朝時代")
        }
        else if iso_3602_strict.to_lowercase() == "nanbokutyô" {
            Period::from_kanji("南北朝時代")
        }
        else if iso_3602_strict.to_lowercase() == "sengoku zidai" {
            Period::from_kanji("戦国時代")
        }
        else if iso_3602_strict.to_lowercase() == "sengokuzidai" {
            Period::from_kanji("戦国時代")
        }
        else if iso_3602_strict.to_lowercase() == "sengoku" {
            Period::from_kanji("戦国時代")
        }
        else if iso_3602_strict.to_lowercase() == "aduti-momoyama zidai" {
            Period::from_kanji("安土桃山時代")
        }
        else if iso_3602_strict.to_lowercase() == "adutimomoyama zidai" {
            Period::from_kanji("安土桃山時代")
        }
        else if iso_3602_strict.to_lowercase() == "adutimomoyamazidai" {
            Period::from_kanji("安土桃山時代")
        }
        else if iso_3602_strict.to_lowercase() == "aduti-momoyama" {
            Period::from_kanji("安土桃山時代")
        }
        else if iso_3602_strict.to_lowercase() == "adutimomoyama" {
            Period::from_kanji("安土桃山時代")
        }
        else if iso_3602_strict.to_lowercase() == "edo zidai" {
            Period::from_kanji("江戸時代")
        }
        else if iso_3602_strict.to_lowercase() == "edozidai" {
            Period::from_kanji("江戸時代")
        }
        else if iso_3602_strict.to_lowercase() == "edo" {
            Period::from_kanji("江戸時代")
        }
        else if iso_3602_strict.to_lowercase() == "tokugawa zidai" {
            Period::from_kanji("徳川時代")
        }
        else if iso_3602_strict.to_lowercase() == "tokugawazidai" {
            Period::from_kanji("徳川時代")
        }
        else if iso_3602_strict.to_lowercase() == "tokugawa" {
            Period::from_kanji("徳川時代")
        }
        else if iso_3602_strict.to_lowercase() == "meizi zidai" {
            Period::from_kanji("明治時代")
        }
        else if iso_3602_strict.to_lowercase() == "meizizidai" {
            Period::from_kanji("明治時代")
        }
        else if iso_3602_strict.to_lowercase() == "meizi" {
            Period::from_kanji("明治時代")
        }
        else if iso_3602_strict.to_lowercase() == "taisyô zidai" {
            Period::from_kanji("大正時代")
        }
        else if iso_3602_strict.to_lowercase() == "taisyôzidai" {
            Period::from_kanji("大正時代")
        }
        else if iso_3602_strict.to_lowercase() == "taisyô" {
            Period::from_kanji("大正時代")
        }
        else if iso_3602_strict.to_lowercase() == "syôwa zidai" {
            Period::from_kanji("昭和時代")
        }
        else if iso_3602_strict.to_lowercase() == "syôwazidai" {
            Period::from_kanji("昭和時代")
        }
        else if iso_3602_strict.to_lowercase() == "syôwa" {
            Period::from_kanji("昭和時代")
        }
        else if iso_3602_strict.to_lowercase() == "heisei zidai" {
            Period::from_kanji("平成時代")
        }
        else if iso_3602_strict.to_lowercase() == "heiseizidai" {
            Period::from_kanji("平成時代")
        }
        else if iso_3602_strict.to_lowercase() == "heisei" {
            Period::from_kanji("平成時代")
        }
        else {
            None
        }
    }

    pub fn from_translation(translation: &str) -> Option<Self> {
        if translation.to_lowercase() == "asuka period" {
            Period::from_kanji("飛鳥時代")
        }
        else if translation.to_lowercase() == "nara period" {
            Period::from_kanji("奈良時代")
        }
        else if translation.to_lowercase() == "heian period" {
            Period::from_kanji("平安時代")
        }
        else if translation.to_lowercase() == "kamakura period" {
            Period::from_kanji("鎌倉時代")
        }
        else if translation.to_lowercase() == "kenmu restoration" {
            Period::from_kanji("建武の新政")
        }
        else if translation.to_lowercase() == "muromachi period" {
            Period::from_kanji("室町時代")
        }
        else if translation.to_lowercase() == "ashikaga period" {
            Period::from_kanji("足利時代")
        }
        else if translation.to_lowercase() == "northern and southern courts period" {
            Period::from_kanji("南北朝時代")
        }
        else if translation.to_lowercase() == "warring states period" {
            Period::from_kanji("戦国時代")
        }
        else if translation.to_lowercase() == "sengoku period" {
            Period::from_kanji("戦国時代")
        }
        else if translation.to_lowercase() == "azuchi-momoyama period" {
            Period::from_kanji("安土桃山時代")
        }
        else if translation.to_lowercase() == "edo period" {
            Period::from_kanji("江戸時代")
        }
        else if translation.to_lowercase() == "tokugawa period" {
            Period::from_kanji("徳川時代")
        }
        else if translation.to_lowercase() == "meiji period" {
            Period::from_kanji("明治時代")
        }
        else if translation.to_lowercase() == "meiji era" {
            Period::from_kanji("明治時代")
        }
        else if translation.to_lowercase() == "taisho period" {
            Period::from_kanji("大正時代")
        }
        else if translation.to_lowercase() == "taisho era" {
            Period::from_kanji("大正時代")
        }
        else if translation.to_lowercase() == "showa period" {
            Period::from_kanji("昭和時代")
        }
        else if translation.to_lowercase() == "showa era" {
            Period::from_kanji("昭和時代")
        }
        else if translation.to_lowercase() == "heisei period" {
            Period::from_kanji("平成時代")
        }
        else if translation.to_lowercase() == "heisei era" {
            Period::from_kanji("平成時代")
        }
        else {
            None
        }
    }

    fn from_year(year: u32) -> Vec<Self> {
        if year < 538 {
            vec![]
        }
        else if year < 710 {
            vec![Period::from_kanji("飛鳥時代").unwrap()]
        }
        else if year == 710 {
            vec![Period::from_kanji("飛鳥時代").unwrap(), Period::from_kanji("奈良時代").unwrap()]
        }
        else if year < 794 {
            vec![Period::from_kanji("奈良時代").unwrap()]
        }
        else if year == 794 {
            vec![Period::from_kanji("奈良時代").unwrap(), Period::from_kanji("平安時代").unwrap()]
        }
        else if year < 1185 {
            vec![Period::from_kanji("平安時代").unwrap()]
        }
        else if year == 1185 {
            vec![Period::from_kanji("平安時代").unwrap(), Period::from_kanji("鎌倉時代").unwrap()]
        }
        else if year < 1333 {
            vec![Period::from_kanji("鎌倉時代").unwrap()]
        }
        else if year == 1333 {
            vec![Period::from_kanji("鎌倉時代").unwrap(), Period::from_kanji("建武の新政").unwrap()]
        }
        else if year < 1336 {
            vec![Period::from_kanji("建武の新政").unwrap()]
        }
        else if year == 1336 {
            vec![Period::from_kanji("建武の新政").unwrap(), Period::from_kanji("室町時代").unwrap(), Period::from_kanji("足利時代").unwrap(), Period::from_kanji("南北朝時代").unwrap()]
        }
        else if year <= 1392 {
            vec![Period::from_kanji("室町時代").unwrap(), Period::from_kanji("足利時代").unwrap(), Period::from_kanji("南北朝時代").unwrap()]
        }
        else if year < 1467 {
            vec![Period::from_kanji("室町時代").unwrap(), Period::from_kanji("足利時代").unwrap()]
        }
        else if year <= 1568 {
            vec![Period::from_kanji("室町時代").unwrap(), Period::from_kanji("足利時代").unwrap(), Period::from_kanji("戦国時代").unwrap()]
        }
        else if year < 1573 {
            vec![Period::from_kanji("室町時代").unwrap(), Period::from_kanji("足利時代").unwrap()]
        }
        else if year == 1573 {
            vec![Period::from_kanji("室町時代").unwrap(), Period::from_kanji("足利時代").unwrap(), Period::from_kanji("安土桃山時代").unwrap()]
        }
        else if year < 1603 {
            vec![Period::from_kanji("安土桃山時代").unwrap()]
        }
        else if year == 1603 {
            vec![Period::from_kanji("安土桃山時代").unwrap(), Period::from_kanji("江戸時代").unwrap(), Period::from_kanji("徳川時代").unwrap()]
        }
        else if year < 1868 {
            vec![Period::from_kanji("江戸時代").unwrap(), Period::from_kanji("徳川時代").unwrap()]
        }
        else if year == 1868 {
            vec![Period::from_kanji("江戸時代").unwrap(), Period::from_kanji("徳川時代").unwrap(), Period::from_kanji("明治時代").unwrap()]
        }
        else if year < 1912 {
            vec![Period::from_kanji("明治時代").unwrap()]
        }
        else if year == 1912 {
            vec![Period::from_kanji("明治時代").unwrap(), Period::from_kanji("大正時代").unwrap()]
        }
        else if year < 1926 {
            vec![Period::from_kanji("大正時代").unwrap()]
        }
        else if year == 1926 {
            vec![Period::from_kanji("大正時代").unwrap(), Period::from_kanji("昭和時代").unwrap()]
        }
        else if year < 1989 {
            vec![Period::from_kanji("昭和時代").unwrap()]
        }
        else if year == 1989 {
            vec![Period::from_kanji("昭和時代").unwrap(), Period::from_kanji("平成時代").unwrap()]
        }
        else if year <= 2019 {
            vec![Period::from_kanji("平成時代").unwrap()]
        }
        else {
            vec![]
        }
    }

    fn from_era_kanji(era: &str) -> Vec<Self> {
        unimplemented!()
    }

    pub fn from_era(era: Era) -> Vec<Self> {
        let year = era.start_georgian.year() as u32;

        Period::from_year(year)
    }
}