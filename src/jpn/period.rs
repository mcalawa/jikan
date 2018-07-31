use std::collections::HashMap;
pub use jpn::era::Era;

lazy_static! {
    static ref PERIODS: HashMap<&'static str, Period> = {
        let mut m = HashMap::new();
        m.insert("asuka", Period::new("Asuka jidai", "飛鳥時代", "あすかじだい", "Asuka jidai", "Asuka zidai", "Asuka zidai", "Asuka period", 538, 710));
        m.insert("asukajidai", Period::new("Asuka jidai", "飛鳥時代", "あすかじだい", "Asuka jidai", "Asuka zidai", "Asuka zidai", "Asuka period", 538, 710));
        m.insert("asukazidai", Period::new("Asuka jidai", "飛鳥時代", "あすかじだい", "Asuka jidai", "Asuka zidai", "Asuka zidai", "Asuka period", 538, 710));
        m.insert("asukaperiod", Period::new("Asuka jidai", "飛鳥時代", "あすかじだい", "Asuka jidai", "Asuka zidai", "Asuka zidai", "Asuka period", 538, 710));
        m.insert("飛鳥", Period::new("Asuka jidai", "飛鳥時代", "あすかじだい", "Asuka jidai", "Asuka zidai", "Asuka zidai", "Asuka period", 538, 710));
        m.insert("飛鳥時代", Period::new("Asuka jidai", "飛鳥時代", "あすかじだい", "Asuka jidai", "Asuka zidai", "Asuka zidai", "Asuka period", 538, 710));
        m.insert("あすか", Period::new("Asuka jidai", "飛鳥時代", "あすかじだい", "Asuka jidai", "Asuka zidai", "Asuka zidai", "Asuka period", 538, 710));
        m.insert("あすかじだい", Period::new("Asuka jidai", "飛鳥時代", "あすかじだい", "Asuka jidai", "Asuka zidai", "Asuka zidai", "Asuka period", 538, 710));
        m.insert("nara", Period::new("Nara jidai", "奈良時代", "ならじだい", "Nara jidai", "Nara zidai", "Nara zidai", "Nara period", 710, 794));
        m.insert("narajidai", Period::new("Nara jidai", "奈良時代", "ならじだい", "Nara jidai", "Nara zidai", "Nara zidai", "Nara period", 710, 794));
        m.insert("narazidai", Period::new("Nara jidai", "奈良時代", "ならじだい", "Nara jidai", "Nara zidai", "Nara zidai", "Nara period", 710, 794));
        m.insert("奈良", Period::new("Nara jidai", "奈良時代", "ならじだい", "Nara jidai", "Nara zidai", "Nara zidai", "Nara period", 710, 794));
        m.insert("奈良時代", Period::new("Nara jidai", "奈良時代", "ならじだい", "Nara jidai", "Nara zidai", "Nara zidai", "Nara period", 710, 794));
        m.insert("なら", Period::new("Nara jidai", "奈良時代", "ならじだい", "Nara jidai", "Nara zidai", "Nara zidai", "Nara period", 710, 794));
        m.insert("ならじだい", Period::new("Nara jidai", "奈良時代", "ならじだい", "Nara jidai", "Nara zidai", "Nara zidai", "Nara period", 710, 794));
        m.insert("heian", Period::new("Heian jidai", "平安時代", "へいあんじだい", "Heian jidai", "Heian zidai", "Heian zidai", "Heian period", 794, 1185));
        m.insert("heianjidai", Period::new("Heian jidai", "平安時代", "へいあんじだい", "Heian jidai", "Heian zidai", "Heian zidai", "Heian period", 794, 1185));
        m.insert("heianzidai", Period::new("Heian jidai", "平安時代", "へいあんじだい", "Heian jidai", "Heian zidai", "Heian zidai", "Heian period", 794, 1185));
        m.insert("平安", Period::new("Heian jidai", "平安時代", "へいあんじだい", "Heian jidai", "Heian zidai", "Heian zidai", "Heian period", 794, 1185));
        m.insert("平安時代", Period::new("Heian jidai", "平安時代", "へいあんじだい", "Heian jidai", "Heian zidai", "Heian zidai", "Heian period", 794, 1185));
        m.insert("へいあん", Period::new("Heian jidai", "平安時代", "へいあんじだい", "Heian jidai", "Heian zidai", "Heian zidai", "Heian period", 794, 1185));
        m.insert("へいあんじだい", Period::new("Heian jidai", "平安時代", "へいあんじだい", "Heian jidai", "Heian zidai", "Heian zidai", "Heian period", 794, 1185));
        m.insert("kamakura", Period::new("Kamakura jidai", "鎌倉時代", "かまくらじだい", "Kamakura jidai", "Kamakura zidai", "Kamakura zidai", "Kamakura period", 1185, 1333));
        m.insert("kamakurajidai", Period::new("Kamakura jidai", "鎌倉時代", "かまくらじだい", "Kamakura jidai", "Kamakura zidai", "Kamakura zidai", "Kamakura period", 1185, 1333));
        m.insert("kamakurazidai", Period::new("Kamakura jidai", "鎌倉時代", "かまくらじだい", "Kamakura jidai", "Kamakura zidai", "Kamakura zidai", "Kamakura period", 1185, 1333));
        m.insert("鎌倉", Period::new("Kamakura jidai", "鎌倉時代", "かまくらじだい", "Kamakura jidai", "Kamakura zidai", "Kamakura zidai", "Kamakura period", 1185, 1333));
        m.insert("鎌倉時代", Period::new("Kamakura jidai", "鎌倉時代", "かまくらじだい", "Kamakura jidai", "Kamakura zidai", "Kamakura zidai", "Kamakura period", 1185, 1333));
        m.insert("かまくら", Period::new("Kamakura jidai", "鎌倉時代", "かまくらじだい", "Kamakura jidai", "Kamakura zidai", "Kamakura zidai", "Kamakura period", 1185, 1333));
        m.insert("かまくらじだい", Period::new("Kamakura jidai", "鎌倉時代", "かまくらじだい", "Kamakura jidai", "Kamakura zidai", "Kamakura zidai", "Kamakura period", 1185, 1333));
        m.insert("kenmu", Period::new("Kenmu no shinsei", "建武の新政", "けんむのしんせい", "Kenmu no shinsei", "Kenmu no sinsei", "Kenmu no sinsei", "Kenmu restoration", 1333, 1336));
        m.insert("kenmunoshinsei", Period::new("Kenmu no shinsei", "建武の新政", "けんむのしんせい", "Kenmu no shinsei", "Kenmu no sinsei", "Kenmu no sinsei", "Kenmu restoration", 1333, 1336));
        m.insert("kenmunosinsei", Period::new("Kenmu no shinsei", "建武の新政", "けんむのしんせい", "Kenmu no shinsei", "Kenmu no sinsei", "Kenmu no sinsei", "Kenmu restoration", 1333, 1336));
        m.insert("建武", Period::new("Kenmu no shinsei", "建武の新政", "けんむのしんせい", "Kenmu no shinsei", "Kenmu no sinsei", "Kenmu no sinsei", "Kenmu restoration", 1333, 1336));
        m.insert("建武の新政", Period::new("Kenmu no shinsei", "建武の新政", "けんむのしんせい", "Kenmu no shinsei", "Kenmu no sinsei", "Kenmu no sinsei", "Kenmu restoration", 1333, 1336));
        m.insert("けんむ", Period::new("Kenmu no shinsei", "建武の新政", "けんむのしんせい", "Kenmu no shinsei", "Kenmu no sinsei", "Kenmu no sinsei", "Kenmu restoration", 1333, 1336));
        m.insert("けんむのしんせい", Period::new("Kenmu no shinsei", "建武の新政", "けんむのしんせい", "Kenmu no shinsei", "Kenmu no sinsei", "Kenmu no sinsei", "Kenmu restoration", 1333, 1336));
        m.insert("muromachi", Period::new("Muromachi jidai", "室町時代", "むろまちじだい", "Muromachi jidai", "Muromati zidai", "Muromati zidai", "Muromachi period", 1336, 1573));
        m.insert("muromachijidai", Period::new("Muromachi jidai", "室町時代", "むろまちじだい", "Muromachi jidai", "Muromati zidai", "Muromati zidai", "Muromachi period", 1336, 1573));
        m.insert("muromati", Period::new("Muromachi jidai", "室町時代", "むろまちじだい", "Muromachi jidai", "Muromati zidai", "Muromati zidai", "Muromachi period", 1336, 1573));
        m.insert("muromatizidai", Period::new("Muromachi jidai", "室町時代", "むろまちじだい", "Muromachi jidai", "Muromati zidai", "Muromati zidai", "Muromachi period", 1336, 1573));
        m.insert("室町", Period::new("Muromachi jidai", "室町時代", "むろまちじだい", "Muromachi jidai", "Muromati zidai", "Muromati zidai", "Muromachi period", 1336, 1573));
        m.insert("室町時代", Period::new("Muromachi jidai", "室町時代", "むろまちじだい", "Muromachi jidai", "Muromati zidai", "Muromati zidai", "Muromachi period", 1336, 1573));
        m.insert("むろまち", Period::new("Muromachi jidai", "室町時代", "むろまちじだい", "Muromachi jidai", "Muromati zidai", "Muromati zidai", "Muromachi period", 1336, 1573));
        m.insert("むろまちじだい", Period::new("Muromachi jidai", "室町時代", "むろまちじだい", "Muromachi jidai", "Muromati zidai", "Muromati zidai", "Muromachi period", 1336, 1573));
        m.insert("ashikaga", Period::new("Ashikaga jidai", "足利時代", "あしかがじだい", "Ashikaga jidai", "Asikaga zidai", "Asikaga zidai", "Ashikaga period", 1336, 1573));
        m.insert("ashikagajidai", Period::new("Ashikaga jidai", "足利時代", "あしかがじだい", "Ashikaga jidai", "Asikaga zidai", "Asikaga zidai", "Ashikaga period", 1336, 1573));
        m.insert("asikaga", Period::new("Ashikaga jidai", "足利時代", "あしかがじだい", "Ashikaga jidai", "Asikaga zidai", "Asikaga zidai", "Ashikaga period", 1336, 1573));
        m.insert("asikagazidai", Period::new("Ashikaga jidai", "足利時代", "あしかがじだい", "Ashikaga jidai", "Asikaga zidai", "Asikaga zidai", "Ashikaga period", 1336, 1573));
        m.insert("足利", Period::new("Ashikaga jidai", "足利時代", "あしかがじだい", "Ashikaga jidai", "Asikaga zidai", "Asikaga zidai", "Ashikaga period", 1336, 1573));
        m.insert("足利時代", Period::new("Ashikaga jidai", "足利時代", "あしかがじだい", "Ashikaga jidai", "Asikaga zidai", "Asikaga zidai", "Ashikaga period", 1336, 1573));
        m.insert("あしかが", Period::new("Ashikaga jidai", "足利時代", "あしかがじだい", "Ashikaga jidai", "Asikaga zidai", "Asikaga zidai", "Ashikaga period", 1336, 1573));
        m.insert("あしかがじだい", Period::new("Ashikaga jidai", "足利時代", "あしかがじだい", "Ashikaga jidai", "Asikaga zidai", "Asikaga zidai", "Ashikaga period", 1336, 1573));
        m.insert("nanboku", Period::new("Nanboku-chou jidai", "南北朝時代", "なんぼくちょうじだい", "Nanboku-chō jidai", "Nanboku-tyô zidai", "Nanboku-tyô zidai", "Northern and Southern Courts period", 1336, 1392));
        m.insert("nanbokuchou", Period::new("Nanboku-chou jidai", "南北朝時代", "なんぼくちょうじだい", "Nanboku-chō jidai", "Nanboku-tyô zidai", "Nanboku-tyô zidai", "Northern and Southern Courts period", 1336, 1392));
        m.insert("nanbokuchoujidai", Period::new("Nanboku-chou jidai", "南北朝時代", "なんぼくちょうじだい", "Nanboku-chō jidai", "Nanboku-tyô zidai", "Nanboku-tyô zidai", "Northern and Southern Courts period", 1336, 1392));
        m.insert("nanbokuchō", Period::new("Nanboku-chou jidai", "南北朝時代", "なんぼくちょうじだい", "Nanboku-chō jidai", "Nanboku-tyô zidai", "Nanboku-tyô zidai", "Northern and Southern Courts period", 1336, 1392));
        m.insert("nanbokuchōjidai", Period::new("Nanboku-chou jidai", "南北朝時代", "なんぼくちょうじだい", "Nanboku-chō jidai", "Nanboku-tyô zidai", "Nanboku-tyô zidai", "Northern and Southern Courts period", 1336, 1392));
        m.insert("nanbokutyô", Period::new("Nanboku-chou jidai", "南北朝時代", "なんぼくちょうじだい", "Nanboku-chō jidai", "Nanboku-tyô zidai", "Nanboku-tyô zidai", "Northern and Southern Courts period", 1336, 1392));
        m.insert("nanbokutyôzidai", Period::new("Nanboku-chou jidai", "南北朝時代", "なんぼくちょうじだい", "Nanboku-chō jidai", "Nanboku-tyô zidai", "Nanboku-tyô zidai", "Northern and Southern Courts period", 1336, 1392));
        m.insert("northern", Period::new("Nanboku-chou jidai", "南北朝時代", "なんぼくちょうじだい", "Nanboku-chō jidai", "Nanboku-tyô zidai", "Nanboku-tyô zidai", "Northern and Southern Courts period", 1336, 1392));
        m.insert("south", Period::new("Nanboku-chou jidai", "南北朝時代", "なんぼくちょうじだい", "Nanboku-chō jidai", "Nanboku-tyô zidai", "Nanboku-tyô zidai", "Northern and Southern Courts period", 1336, 1392));
        m.insert("南北朝", Period::new("Nanboku-chou jidai", "南北朝時代", "なんぼくちょうじだい", "Nanboku-chō jidai", "Nanboku-tyô zidai", "Nanboku-tyô zidai", "Northern and Southern Courts period", 1336, 1392));
        m.insert("南北朝時代", Period::new("Nanboku-chou jidai", "南北朝時代", "なんぼくちょうじだい", "Nanboku-chō jidai", "Nanboku-tyô zidai", "Nanboku-tyô zidai", "Northern and Southern Courts period", 1336, 1392));
        m.insert("なんぼくちょう", Period::new("Nanboku-chou jidai", "南北朝時代", "なんぼくちょうじだい", "Nanboku-chō jidai", "Nanboku-tyô zidai", "Nanboku-tyô zidai", "Northern and Southern Courts period", 1336, 1392));
        m.insert("なんぼくちょうじだい", Period::new("Nanboku-chou jidai", "南北朝時代", "なんぼくちょうじだい", "Nanboku-chō jidai", "Nanboku-tyô zidai", "Nanboku-tyô zidai", "Northern and Southern Courts period", 1336, 1392));
        m.insert("sengoku", Period::new("Sengoku jidai", "戦国時代", "せんごくじだい", "Sengoku jidai", "Sengoku zidai", "Sengoku zidai", "Warring States period", 1467, 1568));
        m.insert("sengokujidai", Period::new("Sengoku jidai", "戦国時代", "せんごくじだい", "Sengoku jidai", "Sengoku zidai", "Sengoku zidai", "Warring States period", 1467, 1568));
        m.insert("sengokuzidai", Period::new("Sengoku jidai", "戦国時代", "せんごくじだい", "Sengoku jidai", "Sengoku zidai", "Sengoku zidai", "Warring States period", 1467, 1568));
        m.insert("warring", Period::new("Sengoku jidai", "戦国時代", "せんごくじだい", "Sengoku jidai", "Sengoku zidai", "Sengoku zidai", "Warring States period", 1467, 1568));
        m.insert("戦国", Period::new("Sengoku jidai", "戦国時代", "せんごくじだい", "Sengoku jidai", "Sengoku zidai", "Sengoku zidai", "Warring States period", 1467, 1568));
        m.insert("戦国時代", Period::new("Sengoku jidai", "戦国時代", "せんごくじだい", "Sengoku jidai", "Sengoku zidai", "Sengoku zidai", "Warring States period", 1467, 1568));
        m.insert("せんごく", Period::new("Sengoku jidai", "戦国時代", "せんごくじだい", "Sengoku jidai", "Sengoku zidai", "Sengoku zidai", "Warring States period", 1467, 1568));
        m.insert("せんごくじだい", Period::new("Sengoku jidai", "戦国時代", "せんごくじだい", "Sengoku jidai", "Sengoku zidai", "Sengoku zidai", "Warring States period", 1467, 1568));
        m.insert("aduchi", Period::new("Aduchi-Momoyama jidai", "安土桃山時代", "あづちももやまじだい", "Azuchi-Momoyama jidai", "Azuti-Momoyama zidai", "Aduti-Momoyama zidai", "Azuchi-Momoyama period", 1573, 1603));
        m.insert("aduchimomoyama", Period::new("Aduchi-Momoyama jidai", "安土桃山時代", "あづちももやまじだい", "Azuchi-Momoyama jidai", "Azuti-Momoyama zidai", "Aduti-Momoyama zidai", "Azuchi-Momoyama period", 1573, 1603));
        m.insert("aduchimomoyamajidai", Period::new("Aduchi-Momoyama jidai", "安土桃山時代", "あづちももやまじだい", "Azuchi-Momoyama jidai", "Azuti-Momoyama zidai", "Aduti-Momoyama zidai", "Azuchi-Momoyama period", 1573, 1603));
        m.insert("azuchi", Period::new("Aduchi-Momoyama jidai", "安土桃山時代", "あづちももやまじだい", "Azuchi-Momoyama jidai", "Azuti-Momoyama zidai", "Aduti-Momoyama zidai", "Azuchi-Momoyama period", 1573, 1603));
        m.insert("azuchimomoyama", Period::new("Aduchi-Momoyama jidai", "安土桃山時代", "あづちももやまじだい", "Azuchi-Momoyama jidai", "Azuti-Momoyama zidai", "Aduti-Momoyama zidai", "Azuchi-Momoyama period", 1573, 1603));
        m.insert("azuchimomoyamajidai", Period::new("Aduchi-Momoyama jidai", "安土桃山時代", "あづちももやまじだい", "Azuchi-Momoyama jidai", "Azuti-Momoyama zidai", "Aduti-Momoyama zidai", "Azuchi-Momoyama period", 1573, 1603));
        m.insert("azuti", Period::new("Aduchi-Momoyama jidai", "安土桃山時代", "あづちももやまじだい", "Azuchi-Momoyama jidai", "Azuti-Momoyama zidai", "Aduti-Momoyama zidai", "Azuchi-Momoyama period", 1573, 1603));
        m.insert("azutimomoyama", Period::new("Aduchi-Momoyama jidai", "安土桃山時代", "あづちももやまじだい", "Azuchi-Momoyama jidai", "Azuti-Momoyama zidai", "Aduti-Momoyama zidai", "Azuchi-Momoyama period", 1573, 1603));
        m.insert("azutimomoyamazidai", Period::new("Aduchi-Momoyama jidai", "安土桃山時代", "あづちももやまじだい", "Azuchi-Momoyama jidai", "Azuti-Momoyama zidai", "Aduti-Momoyama zidai", "Azuchi-Momoyama period", 1573, 1603));
        m.insert("aduti", Period::new("Aduchi-Momoyama jidai", "安土桃山時代", "あづちももやまじだい", "Azuchi-Momoyama jidai", "Azuti-Momoyama zidai", "Aduti-Momoyama zidai", "Azuchi-Momoyama period", 1573, 1603));
        m.insert("adutimomoyama", Period::new("Aduchi-Momoyama jidai", "安土桃山時代", "あづちももやまじだい", "Azuchi-Momoyama jidai", "Azuti-Momoyama zidai", "Aduti-Momoyama zidai", "Azuchi-Momoyama period", 1573, 1603));
        m.insert("adutimomoyamazidai", Period::new("Aduchi-Momoyama jidai", "安土桃山時代", "あづちももやまじだい", "Azuchi-Momoyama jidai", "Azuti-Momoyama zidai", "Aduti-Momoyama zidai", "Azuchi-Momoyama period", 1573, 1603));
        m.insert("安土桃山", Period::new("Aduchi-Momoyama jidai", "安土桃山時代", "あづちももやまじだい", "Azuchi-Momoyama jidai", "Azuti-Momoyama zidai", "Aduti-Momoyama zidai", "Azuchi-Momoyama period", 1573, 1603));
        m.insert("安土桃山時代", Period::new("Aduchi-Momoyama jidai", "安土桃山時代", "あづちももやまじだい", "Azuchi-Momoyama jidai", "Azuti-Momoyama zidai", "Aduti-Momoyama zidai", "Azuchi-Momoyama period", 1573, 1603));
        m.insert("あづちももやま", Period::new("Aduchi-Momoyama jidai", "安土桃山時代", "あづちももやまじだい", "Azuchi-Momoyama jidai", "Azuti-Momoyama zidai", "Aduti-Momoyama zidai", "Azuchi-Momoyama period", 1573, 1603));
        m.insert("あづちももやまじだい", Period::new("Aduchi-Momoyama jidai", "安土桃山時代", "あづちももやまじだい", "Azuchi-Momoyama jidai", "Azuti-Momoyama zidai", "Aduti-Momoyama zidai", "Azuchi-Momoyama period", 1573, 1603));
        m.insert("edo", Period::new("Edo jidai", "江戸時代", "えどじだい", "Edo jidai", "Edo zidai", "Edo zidai", "Edo period", 1603, 1868));
        m.insert("edojidai", Period::new("Edo jidai", "江戸時代", "えどじだい", "Edo jidai", "Edo zidai", "Edo zidai", "Edo period", 1603, 1868));
        m.insert("edozidai", Period::new("Edo jidai", "江戸時代", "えどじだい", "Edo jidai", "Edo zidai", "Edo zidai", "Edo period", 1603, 1868));
        m.insert("江戸", Period::new("Edo jidai", "江戸時代", "えどじだい", "Edo jidai", "Edo zidai", "Edo zidai", "Edo period", 1603, 1868));
        m.insert("江戸時代", Period::new("Edo jidai", "江戸時代", "えどじだい", "Edo jidai", "Edo zidai", "Edo zidai", "Edo period", 1603, 1868));
        m.insert("えど", Period::new("Edo jidai", "江戸時代", "えどじだい", "Edo jidai", "Edo zidai", "Edo zidai", "Edo period", 1603, 1868));
        m.insert("えどじだい", Period::new("Edo jidai", "江戸時代", "えどじだい", "Edo jidai", "Edo zidai", "Edo zidai", "Edo period", 1603, 1868));
        m.insert("tokugawa", Period::new("Tokugawa jidai", "徳川時代", "とくがわじだい", "Tokugawa jidai", "Tokugawa zidai", "Tokugawa zidai", "Tokugawa period", 1603, 1868));
        m.insert("tokugawajidai", Period::new("Tokugawa jidai", "徳川時代", "とくがわじだい", "Tokugawa jidai", "Tokugawa zidai", "Tokugawa zidai", "Tokugawa period", 1603, 1868));
        m.insert("tokugawazidai", Period::new("Tokugawa jidai", "徳川時代", "とくがわじだい", "Tokugawa jidai", "Tokugawa zidai", "Tokugawa zidai", "Tokugawa period", 1603, 1868));
        m.insert("徳川", Period::new("Tokugawa jidai", "徳川時代", "とくがわじだい", "Tokugawa jidai", "Tokugawa zidai", "Tokugawa zidai", "Tokugawa period", 1603, 1868));
        m.insert("徳川時代", Period::new("Tokugawa jidai", "徳川時代", "とくがわじだい", "Tokugawa jidai", "Tokugawa zidai", "Tokugawa zidai", "Tokugawa period", 1603, 1868));
        m.insert("とくがわ", Period::new("Tokugawa jidai", "徳川時代", "とくがわじだい", "Tokugawa jidai", "Tokugawa zidai", "Tokugawa zidai", "Tokugawa period", 1603, 1868));
        m.insert("とくがわじだい", Period::new("Tokugawa jidai", "徳川時代", "とくがわじだい", "Tokugawa jidai", "Tokugawa zidai", "Tokugawa zidai", "Tokugawa period", 1603, 1868));
        m.insert("meiji", Period::new("Meiji jidai", "明治時代", "めいじじだい", "Meiji jidai", "Meizi zidai", "Meizi zidai", "Meiji period", 1868, 1912));
        m.insert("meijijidai", Period::new("Meiji jidai", "明治時代", "めいじじだい", "Meiji jidai", "Meizi zidai", "Meizi zidai", "Meiji period", 1868, 1912));
        m.insert("meizi", Period::new("Meiji jidai", "明治時代", "めいじじだい", "Meiji jidai", "Meizi zidai", "Meizi zidai", "Meiji period", 1868, 1912));
        m.insert("meizizidai", Period::new("Meiji jidai", "明治時代", "めいじじだい", "Meiji jidai", "Meizi zidai", "Meizi zidai", "Meiji period", 1868, 1912));
        m.insert("明治", Period::new("Meiji jidai", "明治時代", "めいじじだい", "Meiji jidai", "Meizi zidai", "Meizi zidai", "Meiji period", 1868, 1912));
        m.insert("明治時代", Period::new("Meiji jidai", "明治時代", "めいじじだい", "Meiji jidai", "Meizi zidai", "Meizi zidai", "Meiji period", 1868, 1912));
        m.insert("めいじ", Period::new("Meiji jidai", "明治時代", "めいじじだい", "Meiji jidai", "Meizi zidai", "Meizi zidai", "Meiji period", 1868, 1912));
        m.insert("めいじじだい", Period::new("Meiji jidai", "明治時代", "めいじじだい", "Meiji jidai", "Meizi zidai", "Meizi zidai", "Meiji period", 1868, 1912));
        m.insert("㍾", Period::new("Meiji jidai", "明治時代", "めいじじだい", "Meiji jidai", "Meizi zidai", "Meizi zidai", "Meiji period", 1868, 1912));
        m.insert("taisho", Period::new("Taishou jidai", "大正時代", "たいしょうじだい", "Taishō jidai", "Taisyô zidai", "Taisyô zidai", "Taisho period", 1912, 1926));
        m.insert("taishou", Period::new("Taishou jidai", "大正時代", "たいしょうじだい", "Taishō jidai", "Taisyô zidai", "Taisyô zidai", "Taisho period", 1912, 1926));
        m.insert("taishoujidai", Period::new("Taishou jidai", "大正時代", "たいしょうじだい", "Taishō jidai", "Taisyô zidai", "Taisyô zidai", "Taisho period", 1912, 1926));
        m.insert("taisyo", Period::new("Taishou jidai", "大正時代", "たいしょうじだい", "Taishō jidai", "Taisyô zidai", "Taisyô zidai", "Taisho period", 1912, 1926));
        m.insert("taisyou", Period::new("Taishou jidai", "大正時代", "たいしょうじだい", "Taishō jidai", "Taisyô zidai", "Taisyô zidai", "Taisho period", 1912, 1926));
        m.insert("taisyoujidai", Period::new("Taishou jidai", "大正時代", "たいしょうじだい", "Taishō jidai", "Taisyô zidai", "Taisyô zidai", "Taisho period", 1912, 1926));
        m.insert("taishō", Period::new("Taishou jidai", "大正時代", "たいしょうじだい", "Taishō jidai", "Taisyô zidai", "Taisyô zidai", "Taisho period", 1912, 1926));
        m.insert("taishōjidai", Period::new("Taishou jidai", "大正時代", "たいしょうじだい", "Taishō jidai", "Taisyô zidai", "Taisyô zidai", "Taisho period", 1912, 1926));
        m.insert("taisyô", Period::new("Taishou jidai", "大正時代", "たいしょうじだい", "Taishō jidai", "Taisyô zidai", "Taisyô zidai", "Taisho period", 1912, 1926));
        m.insert("taisyôzidai", Period::new("Taishou jidai", "大正時代", "たいしょうじだい", "Taishō jidai", "Taisyô zidai", "Taisyô zidai", "Taisho period", 1912, 1926));
        m.insert("大正", Period::new("Taishou jidai", "大正時代", "たいしょうじだい", "Taishō jidai", "Taisyô zidai", "Taisyô zidai", "Taisho period", 1912, 1926));
        m.insert("大正時代", Period::new("Taishou jidai", "大正時代", "たいしょうじだい", "Taishō jidai", "Taisyô zidai", "Taisyô zidai", "Taisho period", 1912, 1926));
        m.insert("たいしょう", Period::new("Taishou jidai", "大正時代", "たいしょうじだい", "Taishō jidai", "Taisyô zidai", "Taisyô zidai", "Taisho period", 1912, 1926));
        m.insert("たいしょうじだい", Period::new("Taishou jidai", "大正時代", "たいしょうじだい", "Taishō jidai", "Taisyô zidai", "Taisyô zidai", "Taisho period", 1912, 1926));
        m.insert("㍽", Period::new("Taishou jidai", "大正時代", "たいしょうじだい", "Taishō jidai", "Taisyô zidai", "Taisyô zidai", "Taisho period", 1912, 1926));
        m.insert("shouwa", Period::new("Shouwa jidai", "昭和時代", "しょうわじだい", "Shōwa jidai", "Syôwa zidai", "Syôwa zidai", "Showa period", 1926, 1989));
        m.insert("shouwajidai", Period::new("Shouwa jidai", "昭和時代", "しょうわじだい", "Shōwa jidai", "Syôwa zidai", "Syôwa zidai", "Showa period", 1926, 1989));
        m.insert("syouwa", Period::new("Shouwa jidai", "昭和時代", "しょうわじだい", "Shōwa jidai", "Syôwa zidai", "Syôwa zidai", "Showa period", 1926, 1989));
        m.insert("syouwajidai", Period::new("Shouwa jidai", "昭和時代", "しょうわじだい", "Shōwa jidai", "Syôwa zidai", "Syôwa zidai", "Showa period", 1926, 1989));
        m.insert("showa", Period::new("Shouwa jidai", "昭和時代", "しょうわじだい", "Shōwa jidai", "Syôwa zidai", "Syôwa zidai", "Showa period", 1926, 1989));
        m.insert("syowa", Period::new("Shouwa jidai", "昭和時代", "しょうわじだい", "Shōwa jidai", "Syôwa zidai", "Syôwa zidai", "Showa period", 1926, 1989));
        m.insert("shōwa", Period::new("Shouwa jidai", "昭和時代", "しょうわじだい", "Shōwa jidai", "Syôwa zidai", "Syôwa zidai", "Showa period", 1926, 1989));
        m.insert("shōwajidai", Period::new("Shouwa jidai", "昭和時代", "しょうわじだい", "Shōwa jidai", "Syôwa zidai", "Syôwa zidai", "Showa period", 1926, 1989));
        m.insert("syôwa", Period::new("Shouwa jidai", "昭和時代", "しょうわじだい", "Shōwa jidai", "Syôwa zidai", "Syôwa zidai", "Showa period", 1926, 1989));
        m.insert("syôwazidai", Period::new("Shouwa jidai", "昭和時代", "しょうわじだい", "Shōwa jidai", "Syôwa zidai", "Syôwa zidai", "Showa period", 1926, 1989));
        m.insert("昭和", Period::new("Shouwa jidai", "昭和時代", "しょうわじだい", "Shōwa jidai", "Syôwa zidai", "Syôwa zidai", "Showa period", 1926, 1989));
        m.insert("昭和時代", Period::new("Shouwa jidai", "昭和時代", "しょうわじだい", "Shōwa jidai", "Syôwa zidai", "Syôwa zidai", "Showa period", 1926, 1989));
        m.insert("しょうわ", Period::new("Shouwa jidai", "昭和時代", "しょうわじだい", "Shōwa jidai", "Syôwa zidai", "Syôwa zidai", "Showa period", 1926, 1989));
        m.insert("しょうわじだい", Period::new("Shouwa jidai", "昭和時代", "しょうわじだい", "Shōwa jidai", "Syôwa zidai", "Syôwa zidai", "Showa period", 1926, 1989));
        m.insert("㍼", Period::new("Shouwa jidai", "昭和時代", "しょうわじだい", "Shōwa jidai", "Syôwa zidai", "Syôwa zidai", "Showa period", 1926, 1989));
        m.insert("heisei", Period::new("Heisei jidai", "平成時代", "へいせいじだい", "Heisei jidai", "Heisei zidai", "Heisei zidai", "Heisei period", 1989, 2019));
        m.insert("heiseijidai", Period::new("Heisei jidai", "平成時代", "へいせいじだい", "Heisei jidai", "Heisei zidai", "Heisei zidai", "Heisei period", 1989, 2019));
        m.insert("heiseizidai", Period::new("Heisei jidai", "平成時代", "へいせいじだい", "Heisei jidai", "Heisei zidai", "Heisei zidai", "Heisei period", 1989, 2019));
        m.insert("平成", Period::new("Heisei jidai", "平成時代", "へいせいじだい", "Heisei jidai", "Heisei zidai", "Heisei zidai", "Heisei period", 1989, 2019));
        m.insert("平成時代", Period::new("Heisei jidai", "平成時代", "へいせいじだい", "Heisei jidai", "Heisei zidai", "Heisei zidai", "Heisei period", 1989, 2019));
        m.insert("へいせい", Period::new("Heisei jidai", "平成時代", "へいせいじだい", "Heisei jidai", "Heisei zidai", "Heisei zidai", "Heisei period", 1989, 2019));
        m.insert("へいせいじだい", Period::new("Heisei jidai", "平成時代", "へいせいじだい", "Heisei jidai", "Heisei zidai", "Heisei zidai", "Heisei period", 1989, 2019));
        m.insert("㍻", Period::new("Heisei jidai", "平成時代", "へいせいじだい", "Heisei jidai", "Heisei zidai", "Heisei zidai", "Heisei period", 1989, 2019));
        m
    };
}

#[derive(Eq, PartialEq, Debug)]
pub struct Period {
    kana_spelling: &'static str,
    kanji: &'static str,
    hiragana: &'static str,
    hepburn: &'static str,
    iso_3602: &'static str,
    iso_3602_strict: &'static str,
    translation: &'static str,
    georgian_start_year: u32,
    georgian_end_year: u32,
}

impl Period {
    pub fn new(kana_spelling: &'static str, kanji: &'static str, hiragana: &'static str, hepburn: &'static str, iso_3602: &'static str, iso_3602_strict: &'static str, translation: &'static str, georgian_start_year: u32, georgian_end_year: u32) -> Self {
        let period = Period {
            kana_spelling: kana_spelling,
            kanji: kanji,
            hiragana: hiragana,
            hepburn: hepburn,
            iso_3602: iso_3602,
            iso_3602_strict: iso_3602_strict,
            translation: translation,
            georgian_start_year: georgian_start_year,
            georgian_end_year: georgian_end_year
        };
        period
    }
    
    pub fn from_str(name: &'static str) -> Option<&'static Self> {
        let formatted_name = name.to_lowercase();
        let mut whitespace_iter = formatted_name.split_whitespace();
        let first_word = whitespace_iter.next().unwrap();
        let mut dash_iter = first_word.split('-');
        let before_dash = dash_iter.next().unwrap();
        PERIODS.get(before_dash)
    }

    fn from_year(year: u32) -> Option<Vec<&'static Self>> {
        if year < 538 {
            None
        }
        else if year < 710 {
            Some(vec![PERIODS.get("飛鳥時代").unwrap()])
        }
        else if year == 710 {
            Some(vec![PERIODS.get("飛鳥時代").unwrap(), PERIODS.get("奈良時代").unwrap()])
        }
        else if year < 794 {
            Some(vec![PERIODS.get("奈良時代").unwrap()])
        }
        else if year == 794 {
            Some(vec![PERIODS.get("奈良時代").unwrap(), PERIODS.get("平安時代").unwrap()])
        }
        else if year < 1185 {
            Some(vec![PERIODS.get("平安時代").unwrap()])
        }
        else if year == 1185 {
            Some(vec![PERIODS.get("平安時代").unwrap(), PERIODS.get("鎌倉時代").unwrap()])
        }
        else if year < 1333 {
            Some(vec![PERIODS.get("鎌倉時代").unwrap()])
        }
        else if year == 1333 {
            Some(vec![PERIODS.get("鎌倉時代").unwrap(), PERIODS.get("建武の新政").unwrap()])
        }
        else if year < 1336 {
            Some(vec![PERIODS.get("建武の新政").unwrap()])
        }
        else if year == 1336 {
            Some(vec![PERIODS.get("建武の新政").unwrap(), PERIODS.get("室町時代").unwrap(), PERIODS.get("足利時代").unwrap(), PERIODS.get("南北朝時代").unwrap()])
        }
        else if year <= 1392 {
            Some(vec![PERIODS.get("室町時代").unwrap(), PERIODS.get("足利時代").unwrap(), PERIODS.get("南北朝時代").unwrap()])
        }
        else if year < 1467 {
            Some(vec![PERIODS.get("室町時代").unwrap(), PERIODS.get("足利時代").unwrap()])
        }
        else if year <= 1568 {
            Some(vec![PERIODS.get("室町時代").unwrap(), PERIODS.get("足利時代").unwrap(), PERIODS.get("戦国時代").unwrap()])
        }
        else if year < 1573 {
            Some(vec![PERIODS.get("室町時代").unwrap(), PERIODS.get("足利時代").unwrap()])
        }
        else if year == 1573 {
            Some(vec![PERIODS.get("室町時代").unwrap(), PERIODS.get("足利時代").unwrap(), PERIODS.get("安土桃山時代").unwrap()])
        }
        else if year < 1603 {
            Some(vec![PERIODS.get("安土桃山時代").unwrap()])
        }
        else if year == 1603 {
            Some(vec![PERIODS.get("安土桃山時代").unwrap(), PERIODS.get("江戸時代").unwrap(), PERIODS.get("徳川時代").unwrap()])
        }
        else if year < 1868 {
            Some(vec![PERIODS.get("江戸時代").unwrap(), PERIODS.get("徳川時代").unwrap()])
        }
        else if year == 1868 {
            Some(vec![PERIODS.get("江戸時代").unwrap(), PERIODS.get("徳川時代").unwrap(), PERIODS.get("明治時代").unwrap()])
        }
        else if year < 1912 {
            Some(vec![PERIODS.get("明治時代").unwrap()])
        }
        else if year == 1912 {
            Some(vec![PERIODS.get("明治時代").unwrap(), PERIODS.get("大正時代").unwrap()])
        }
        else if year < 1926 {
            Some(vec![PERIODS.get("大正時代").unwrap()])
        }
        else if year == 1926 {
            Some(vec![PERIODS.get("大正時代").unwrap(), PERIODS.get("昭和時代").unwrap()])
        }
        else if year < 1989 {
            Some(vec![PERIODS.get("昭和時代").unwrap()])
        }
        else if year == 1989 {
            Some(vec![PERIODS.get("昭和時代").unwrap(), PERIODS.get("平成時代").unwrap()])
        }
        else if year <= 2019 {
            Some(vec![PERIODS.get("平成時代").unwrap()])
        }
        else {
            None
        }
    }

    pub fn from_era(era: Era) -> Option<Vec<&'static Self>> {
        let year = era.georgian_start_year() as u32;

        Period::from_year(year)
    }
}