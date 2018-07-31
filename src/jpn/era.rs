// This is part of Jikan
// See README.md for details

use chrono::prelude::*;
use jpn::period::Period;

#[derive(Clone)]
pub enum Court {
    Unified,
    North,
    South,
    Both,
}

#[derive(Clone)]
pub struct Era {
    kana_spelling: String,
    kanji: String,
    hiragana: String,
    hepburn: String,
    iso_3602: String,
    start_georgian: NaiveDate,
    end_georgian: NaiveDate,
    end_year: u32,
    court: Court,
}

impl Era {
    pub fn new(kana_spelling: String, kanji: String, hiragana: String, hepburn: String, iso_3602: String, start_georgian: NaiveDate, end_georgian: NaiveDate, end_year: u32, court: Court) -> Self {
        let era = Era {
            kana_spelling: kana_spelling,
            kanji: kanji,
            hiragana: hiragana,
            hepburn: hepburn,
            iso_3602: iso_3602,
            start_georgian: start_georgian,
            end_georgian: end_georgian,
            end_year: end_year,
            court: court,
        };
        era
    }

    pub fn get(_era: &str, _period: &str) -> Option<Self> {
        unimplemented!()
    }

    pub fn get_from_period(_era: &str, _period: Period) -> Option<Self> {
        unimplemented!()
    }

    pub fn from_kanji(_kanji: &str) -> Option<Self> {
        unimplemented!()
    }

    pub fn from_georgian(_date: NaiveDate) -> Option<Self> {
        unimplemented!()
    }

    pub fn from_georgian_ymd(_year: u32, _month: u32, _day: u32) -> Option<Self> {
        unimplemented!()
    }

    pub fn georgian_start_year(&self) -> i32 {
        self.start_georgian.year()
    }
}

// const ERAS: [((u32, u32, u32), (u32, u32, u32), (String, String, String, String, String, u32, Court, Option<(u32, u32, u32)>)); ] = [
//     ((645, 7, 20), (650, 3, 25), ("Taika", "大化", "たいか", "Taika", "Taika", 6, Court::Unified, None)), //0
//     ((650, 3, 25), (654, 11, 27), ("Hakuchi", "白雉", "はくち", "Hakuchi", "Hakuti", 5, Court::Unified, None)), //1
//     ((686, 8, 17), (686, 10, 4), ("Shuchou", "朱鳥", "しゅちょう", "Shuchō", "Syutyô", 1, Court::Unified, None)), //2
//     ((701, 5, 7), (704, 6, 20), ("Taihou", "大宝", "たいほう", "Taihō", "Taihô", 4, Court::Unified, None)), //3
//     ((704, 6, 20), (708, 2, 11), ("Keiun", "慶雲", "けいうん", "Keiun", "Keiun", 5, Court::Unified, None)), //4
//     ((708, 2, 11), (715, 10, 7), ("Wadou", "和銅", "わどう", "Wadō", "Wadô", 8, Court::Unified, None)), //5
//     ((715, 10, 7), (717, 12, 28), ("Reiki", "霊亀", "れいき", "Reiki", "Reiki", 3, Court::Unified, None)), //6
//     ((717, 12, 28), (724, 3, 27), ("Yourou", "養老", "ようろう", "Yōrō", "Yôrô", 8 Court::Unified, None)), //7
//     ((724, 3, 27), (729, 9, 6), ("Jinki", "神亀", "じんき", "Jinki", "Zinki", 6, Court::Unified, None)), //8
//     ((729, 9, 6), (749, 5, 9), ("Tenpyou", "天平", "てんぴょう", "Tenpyō", "Tenpyô", 21, Court::Unified, None)), //9
//     ((749, 5, 9), (749, 8, 23), ("Tenpyou-Kanpou", "天平感宝", "てんぴょうかんぽう", "Tenpyō-Kanpō", "Tenpyô-Kanpô", 1, Court::Unified, None)), //10
//     ((749, 8, 23), (757, 9, 10), ("Tenpyou-Shouhou", "天平勝宝", "てんぴょうしょうほう", "Tenpyō-Shōhō", "Tenpyô-Syôhô", 9, Court::Unified, None)), //11
//     ((757, 9, 10), (765, 2, 5), ("Tenpyou-Houji", "天平宝字", "てんぴょうほうじ", "Tenpyō-Hōji", "Tenpyô-Hôji", 9, Court::Unified, None)), //12
//     ((765, 2, 5), (767, 9, 17), ("Tenpyou-Jingo", "天平神護", "てんぴょうじんご", "Tenpyō-Jingo", "Tenpyô-Zingo", 3, Court::Unified, None)), //13
//     ((767, 9, 17), (770, 10, 27), ("Jingo-Keiun", "神護景雲", "じんごけいうん", "Jingo-Keiun", "Zingo-Keiun", 4, Court::Unified, None)), //14
//     ((770, 10, 27), (781, 2, 3), ("Houki", "宝亀", "ほうき", "Hōki", "Hôki", 12, Court::Unified, None)), //15
//     ((781, 2, 3), (782, 10, 4), ("Ten'ou", "天応", "てんおう", "Ten'ō", "Ten'ô", 2, Court::Unified, None)), //16
//     ((782, 10, 4), (806, 6, 12), ("Enryaku", "延暦", "えんりゃく", "Enryaku", "Enryaku", 25, Court::Unified, None)), //17
//     ((806, 6, 12), (810, 10, 24), ("Daidou", "大同", "だいどう", "Daidō", "Daidô", 5, Court::Unified, None)), //18
//     ((810, 10, 24), (824, 2, 12), ("Kounin", "弘仁", "こうにん", "Kōnin", "Kônin", 15, Court::Unified, None)), //19
//     ((824, 2, 12), (834, 2, 18), ("Tenchou", "天長", "てんちょう", "Tenchō", "Tentyô", 11, Court::Unified, None)), //20
//     ((834, 2, 18), (848, 7, 20), ("Jouwa", "承和", "じょうわ", "Jōwa", "Zyôwa", 15, Court::Unified, None)), //21
//     ((848, 7, 20), (851, 6, 5), ("Kashou", "嘉祥", "かしょう", "Kashō", "Kasyô", 4, Court::Unified, None)), //22
//     ((851, 6, 5), (854, 12, 27), ("Ninju", "仁寿", "にんじゅ", "Ninju", "Ninzyu", 4, Court::Unified, None)), //23
//     //edits to do here
//     //edits to do here
//     //edits to do here
//     ((854, 12, 27), (857, 3, 24), ("Saikou", "", "", "Saikō", "", 4, Court::Unified, None)), //24
//     ((857, 3, 24), (859, 5, 24), ("Ten'an", "", "", "Ten'an", "Ten'an", 3, Court::Unified, None)), //25
//     ((859, 5, 24), (877, 6, 5), ("Jougan", "", "", "Jōgan", "", 19, Court::Unified, None)), //26
//     ((877, 6, 5), (885, 3, 15), ("Gangyou", "", "", "Gangyō", "", 9, Court::Unified, None)), //27
//     ((885, 3, 15), (889, 6, 3), ("Ninna", "", "", "Ninna", "Ninna", 5, Court::Unified, None)), //28
//     ((889, 6, 3), (898, 5, 24), ("Kanpyou", "", "", "Kanpyō", "", 10, Court::Unified, None)), //29
//     ((898, 5, 24), (901, 9, 5), ("Shoutai", "", "", "Shōtai", "", 4, Court::Unified, None)), //30
//     ((901, 9, 5), (923, 6, 3), ("Engi", "", "", "Engi", "Engi", 23, Court::Unified, None)), //31
//     ((923, 6, 3), (931, 5, 21), ("Enchou", "", "", "Enchō", "", 9, Court::Unified, None)), //32
//     ((931, 5, 21), (938, 6, 27), ("Jouhei", "", "", "Jōhei", "", 8, None)), //33
//     ((938, 6, 27), (947, 5, 20), ("Tengyou", "", "", "Tengyō", "", 10, Court::Unified, None)), //34
//     ((947, 5, 20), (957, 11, 26), ("Tenryaku", "", "", "Tenryaku", "Tenryaku", 11, Court::Unified, None)), //35
//     ((957, 11, 26), (961, 3, 10), ("Tentoku", "", "", "Tentoku", "Tentoku", 5, Court::Unified, None)), //36
//     ((961, 3, 10), (964, 8, 24), ("Ouwa", "", "", "", "", 4, Court::Unified, None)), //37
//     ((964, 8, 24), (968, 9, 13), ("Kouhou", "", "", "Kōhō", "", 5, Court::Unified, None)), //38
//     ((968, 9, 13), (970, 5, 8), ("Anna", "", "", "Anna", "Anna", 3, Court::Unified, None)), //39
// ];

// const ERAS: [(Vec<&str>, Vec<(Vec<&str>, Era)>); 14] = [
//     (
//         vec![
//             "asuka", 
//             "asuka jidai", 
//             "asukajidai", 
//             "asuka zidai", 
//             "asukazidai", 
//             "asuka period", 
//             "飛鳥時代", 
//             "飛鳥", 
//             "あすかじだい", 
//             "あすか"
//         ], //period
//         vec![
//             (
//                 vec![
//                     "taika", 
//                     "大化", 
//                     "たいか"
//                 ], 
//                 Era::new(
//                     "Taika", 
//                     "大化", 
//                     "たいか", 
//                     "Taika", 
//                     "Taika", 
//                     NaiveDate::from_ymd(645, 7, 20), 
//                     NaiveDate::from_ymd(650, 3, 25), 
//                     6
//                 )
//             ), //0.era[0] Taika
//             (
//                 vec![
//                     "hakuchi",
//                     "hakuti",
//                     "白雉",
//                     "はくち"
//                 ], 
//                 Era::new(
//                     "Hakuchi", 
//                     "白雉", 
//                     "はくち", 
//                     "Hakuchi", 
//                     "Hakuti", 
//                     NaiveDate::from_ymd(650, 3, 25), 
//                     NaiveDate::from_ymd(654, 11, 27), 
//                     5
//                 )
//             ), //0.era[1] Hakuchi
//             (
//                 vec![
//                     "shuchou",
//                     "shuchō",
//                     "syutyô",
//                     "朱鳥",
//                     "しゅちょう"
//                 ],
//                 Era::new(
//                     "Shuchou", 
//                     "朱鳥", 
//                     "しゅちょう", 
//                     "Shuchō", 
//                     "Syutyô", 
//                     NaiveDate::from_ymd(686, 8, 17), 
//                     NaiveDate::from_ymd(686, 10, 4), 
//                     1
//                 )
//             ), //0.era[2] Shuchou
//             (
//                 vec![
//                     "taihou",
//                     "taihō",
//                     "taihô",
//                     "大宝",
//                     "たいほう"
//                 ],
//                 Era::new(
//                     "Taihou", 
//                     "大宝", 
//                     "たいほう", 
//                     "Taihō", 
//                     "Taihô", 
//                     NaiveDate::from_ymd(701, 5, 7), 
//                     NaiveDate::from_ymd(704, 6, 20), 
//                     4
//                 )
//             ), //0.era[3] Taihou
//             (
//                 vec![
//                     "keiun",
//                     "慶雲",
//                     "けいうん"
//                 ],
//                 Era::new(
//                     "Keiun", 
//                     "慶雲", 
//                     "けいうん", 
//                     "Keiun", 
//                     "Keiun", 
//                     NaiveDate::from_ymd(704, 6, 20), 
//                     NaiveDate::from_ymd(708, 2, 11), 
//                     5
//                 )
//             ), //0.era[4] Keiun
//             (
//                 vec![
//                     "wadou",
//                     "wadō",
//                     "wadô",
//                     "和銅",
//                     "わどう"
//                 ],
//                 Era::new(
//                     "Wadou", 
//                     "和銅", 
//                     "わどう", 
//                     "Wadō", 
//                     "Wadô", 
//                     NaiveDate::from_ymd(708, 2, 11), 
//                     NaiveDate::from_ymd(715, 10, 7), 
//                     8
//                 )
//             ) //0.era[5] Wadou
//         ] //0 eras
//     ), //0 Asuka 538-710
//     (
//         vec![
//             "nara", 
//             "nara jidai", 
//             "narajidai", 
//             "nara zidai", 
//             "narazidai", 
//             "nara period", 
//             "奈良時代", 
//             "奈良", 
//             "ならじだい", 
//             "なら"
//         ], //period
//         vec![
//             (
//                 vec![
//                     "reiki",
//                     "霊亀",
//                     "れいき"
//                 ],
//                 Era::new(
//                     "Reiki", 
//                     "霊亀", 
//                     "れいき", 
//                     "Reiki", 
//                     "Reiki", 
//                     NaiveDate::from_ymd(715, 10, 7), 
//                     NaiveDate::from_ymd(717, 12, 28), 
//                     3
//                 )
//             ), //1.era[0] Reiki
//             (
//                 vec![
//                     "yourou",
//                     "yōrō",
//                     "yôrô",
//                     "養老",
//                     "ようろう"
//                 ],
//                 Era::new(
//                     "Yourou", 
//                     "養老", 
//                     "ようろう", 
//                     "Yōrō", 
//                     "Yôrô", 
//                     NaiveDate::from_ymd(717, 12, 28), 
//                     NaiveDate::from_ymd(724, 3, 27), 
//                     8
//                 )
//             ), //1.era[1] Yourou
//             (
//                 vec![
//                     "jinki",
//                     "zinki",
//                     "神亀",
//                     "じんき"
//                 ],
//                 Era::new(
//                     "Jinki", 
//                     "神亀", 
//                     "じんき", 
//                     "Jinki", 
//                     "Zinki", 
//                     NaiveDate::from_ymd(724, 3, 27), 
//                     NaiveDate::from_ymd(729, 9, 6), 
//                     6
//                 )
//             ), //1.era[2] Jinki
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //1.era[3] Tenpyou
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //1.era[4] Tenpyou-Kanpou
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //1.era[5] Tenpyou-Shouhou
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //1.era[6] Tenpyou-Houji
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //1.era[7] Tenpyou-Jingo
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //1.era[8] Jingo-Keiun
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //1.era[9] Houki
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //1.era[10] Ten'ou
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //1.era[11] Enryaku
//         ] //1 eras
//     ), //1 Nara 710-794
//     (
//         vec![
//             "heian", 
//             "heian jidai", 
//             "heianjidai", 
//             "heian zidai", 
//             "heianzidai", 
//             "heian period", 
//             "平安時代", 
//             "平安", 
//             "へいあんじだい", 
//             "へいあん"
//         ], //period
//         vec![
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //2.era[0-87]
//         ] //eras
//     ), //2 Heian 794-1185
//     (
//         vec![
//             "kamakura", 
//             "kamakura jidai", 
//             "kamakurajidai", 
//             "kamakura zidai", 
//             "kamakurazidai", 
//             "kamakura period", 
//             "鎌倉時代", 
//             "鎌倉", 
//             "かまくらじだい", 
//             "かまくら"
//         ], //period
//         vec![
//             (
//                 vec![
//                     "",
//                     "",
//                     ""
//                 ],
//                 Era::new(
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     "", 
//                     NaiveDate::from_ymd(, , ), 
//                     NaiveDate::from_ymd(, , ), 
//                     0
//                 )
//             ), //3.era[0-50]
//         ]
//     ), //3 Kamakura 1185-1333
//     (
//         vec![
//             "kenmu no shinsei", 
//             "kenmunoshinsei", 
//             "kenmu", 
//             "kenmu no sinsei", 
//             "kenmunosinsei", 
//             "kenmu restoration", 
//             "建武の新政", 
//             "建武", 
//             "けんむのしんせい", 
//             "けんむ"
//         ], //period
//         vec![
//             (
//                 vec![
//                     "kenmu"
//                 ],
//                 Era
//             ) //4.era[0] Kenmu
//         ]
//     ), //4 Kenmu no shinsei 1333-1336
//     (
//         vec![
//             "muromachi", 
//             "muromachi jidai", 
//             "muromachijidai", 
//             "muromati", 
//             "muromati zidai", 
//             "muromatizidai", 
//             "muromachi period", 
//             "muromati period", 
//             "室町時代", 
//             "室町", 
//             "むろまちじだい", 
//             "むろまち",
//             "ashikaga", 
//             "ashikaga jidai", 
//             "ashikagajidai", 
//             "asikaga", 
//             "asikaga zidai", 
//             "asikagazidai", 
//             "ashikaga period", 
//             "asikaga period", 
//             "足利時代", 
//             "足利", 
//             "あしかがじだい", 
//             "あしかが"
//         ], //period
//         vec![
//             (
//                 vec![
//                     ""
//                 ],
//                 Era
//             ) //5.era[0-47] 
//         ]
//     ), //5 Muromachi/Ashikaga 1336-1573
//     (
//         vec![
//             "nanboku-chou", 
//             "nanboku-chou jidai", 
//             "nanbokuchou", 
//             "nanbokuchoujidai", 
//             "nanboku-chō", 
//             "nanboku-chō jidai", 
//             "nanbokuchō", 
//             "nanbokuchō jidai", 
//             "nanbokuchōjidai", 
//             "nanboku-tyô", 
//             "nanboku-tyô zidai", 
//             "nanbokutyô", 
//             "nanbokutyôzidai", 
//             "south and north courts period", 
//             "northern and southern courts period", 
//             "南北朝時代", 
//             "南北朝", 
//             "なんぼくちょうじだい", 
//             "なんぼくちょう"
//         ], //periods
//         vec![
//             (
//                 vec![
//                     ""
//                 ],
//                 Era
//             ), //6.era[0-23]
//         ] //6 eras
//     ), //6 Nanboku-chou 1336-1392
//     (
//         vec![
//             "sengoku", 
//             "sengoku jidai", 
//             "sengokujidai", 
//             "sengoku zidai", 
//             "sengokuzidai", 
//             "sengoku period", 
//             "warring states period", 
//             "戦国時代", 
//             "戦国", 
//             "せんごくじだい", 
//             "せんごく"
//         ], //period
//         vec![
//             (
//                 vec![
//                     ""
//                 ],
//                 Era
//             ), //7.era[0-12]
//         ] //7 eras
//     ), //7 Sengoku 1467-1568
//     (
//         vec![
//             "aduchi-momoyama", 
//             "aduchi-momoyama jidai", 
//             "aduchimomoyama", 
//             "aduchimomoyama jidai", 
//             "aduchimomoyamajidai", 
//             "azuchi-momoyama", 
//             "azuchi-momoyama jidai", 
//             "azuchimomoyama", 
//             "azuchimomoyamajidai", 
//             "azuti-momoyama", 
//             "azuti-momoyama zidai", 
//             "azutimomoyama", 
//             "azutimomoyama zidai", 
//             "azutimomoyamazidai", 
//             "aduti-momoyama", 
//             "aduti-momoyama zidai", 
//             "adutimomoyama", 
//             "adutimomoyama zidai", 
//             "adutimomoyamazidai", 
//             "azuchi-momoyama period", 
//             "azuchimomoyama period", 
//             "azuti-momoyama period", 
//             "azutimomoyama period", 
//             "aduti-momoyama period", 
//             "adutimomoyama period", 
//             "安土桃山時代", 
//             "安土桃山", 
//             "あづちももやまじだい", 
//             "あづちももやま"
//         ], //period
//         vec![
//             (
//                 vec![
//                     ""
//                 ],
//                 Era
//             ) //8.era[0-2]
//         ] //8 eras
//     ), //8 Azuchi-Momoyama 1573-1603
//     (
//         vec![
//             "edo", 
//             "edo jidai", 
//             "edojidai", 
//             "edo zidai", 
//             "edozidai", 
//             "edo period", 
//             "江戸時代", 
//             "江戸", 
//             "えどじだい", 
//             "えど",
//             "tokugawa", 
//             "tokugawa jidai", 
//             "tokugawajidai", 
//             "tokugawa zidai", 
//             "tokugawazidai", 
//             "tokugawa period", 
//             "徳川時代", 
//             "徳川", 
//             "とくがわじだい", 
//             "とくがわ"
//         ], //period
//         vec![
//             (
//                 vec![
//                     ""
//                 ],
//                 Era
//             ), //9.era[0-34]
//         ] //eras
//     ), //9 Edo/Tokugawa 1603-1868
//     (
//         vec![
//             "meiji", 
//             "meiji jidai", 
//             "meijijidai", 
//             "meizi", 
//             "meizi zidai", 
//             "meizizidai", 
//             "meiji period", 
//             "meizi period", 
//             "meiji era", 
//             "meizi era", 
//             "明治時代", 
//             "明治", 
//             "めいじじだい", 
//             "めいじ", 
//             "㍾"
//         ], //period
//         vec![
//             (
//                 vec![
//                     "meiji",
//                     "meizi",
//                     "meiji era",
//                     "meizi era",
//                     "明治",
//                     "めいじ",
//                     "㍾"
//                 ],
//                 Era::new(
//                     "Meiji",
//                     "明治",
//                     "めいじ",
//                     "Meiji",
//                     "Meizi",
//                     NaiveDate::from_ymd(1868, 10, 23),
//                     NaiveDate::from_ymd(1912, 7, 29),
//                     45
//                 )
//             ) //10.era[0] Meiji
//         ] //10 eras
//     ), //10 Meiji 1868-1912
//     (
//         vec![
//             "",
//         ], //period
//         vec![
//             (
//                 vec![
//                     ""
//                 ],
//                 Era::new(
//                     "",
//                     "",
//                     "",
//                     "",
//                     "",
//                     NaiveDate::from_ymd(1912, 7, 29),
//                     NaiveDate::from_ymd(, , ),
//                     15
//                 )
//             ) //11.era[0] Taishou
//         ] //11 eras
//     ), //11 Taishou 1912-1926
//     (
//         vec![
//             "",
//         ], //period
//         vec![
//             (
//                 vec![
//                     ""
//                 ],
//                 Era::new(
//                     "",
//                     "",
//                     "",
//                     "",
//                     "",
//                     NaiveDate::from_ymd(, , ),
//                     NaiveDate::from_ymd(, , ),
//                     0
//                 )
//             ) //12.era[0] Shouwa
//         ] //12 eras
//     ), //12 Shouwa 1926-1989
//     (
//         vec![
//             "",
//         ], //period
//         vec![
//             (
//                 vec![
//                     ""
//                 ],
//                 Era::new(
//                     "",
//                     "",
//                     "",
//                     "",
//                     "",
//                     NaiveDate::from_ymd(, , ),
//                     NaiveDate::from_ymd(, , ),
//                     0
//                 )
//             ) //13.era[0] Heisei
//         ] //13 eras
//     ), //13 Heisei 1989-2019
// ];