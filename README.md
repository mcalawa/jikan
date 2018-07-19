# Jikan: Date and Time in Rust Beyond Just Georgian
Jikan is a Rust library for performing conversions of Georgian calendar dates into other calendar systems.

Resources:
* Kang Seonghoon's [Chrono](https://github.com/chronotope/chrono)
* Wikipedia's article on [Japanese era names](https://en.wikipedia.org/wiki/Japanese_era_name), their [list of Japanese era names](https://en.wikipedia.org/wiki/List_of_Japanese_era_names) and articles on individual eras
* The Japanese Wikipedia's [list of Japanese era names](https://ja.wikipedia.org/wiki/%E5%85%83%E5%8F%B7%E4%B8%80%E8%A6%A7_(%E6%97%A5%E6%9C%AC)) supplemented when information was not available in English
* Mattias Schemm's [NengoCalc](http://www.yukikurete.de/nengo_calc.htm)
* Fourmilab's [Calendar Converter](http://www.fourmilab.ch/documents/calendar/)
* The National Diet Library's [the Japanese Calendar](http://www.ndl.go.jp/koyomi/e/index.html)
* The Samurai Archive's [Japanese calendar](http://wiki.samurai-archives.com/index.php?title=Japanese_calendar)
* Wikipedia's articles on the [romanization of Japanese](https://en.wikipedia.org/wiki/Romanization_of_Japanese), [Hepburn romanization](https://en.wikipedia.org/wiki/Hepburn_romanization), [Kurei-shiki romanization](https://en.wikipedia.org/wiki/Kunrei-shiki_romanization), and [Nihon-shiki romanization](https://en.wikipedia.org/wiki/Nihon-shiki_romanization)
* Documentation that provided examples of how solutions were implemented in other languages such as Java's [JapaneseDate](https://docs.oracle.com/javase/8/docs/api/java/time/chrono/JapaneseDate.html) and [JapaneseEra](https://docs.oracle.com/javase/8/docs/api/java/time/chrono/JapaneseEra.html), C#'s [JapaneseCalendar](https://msdn.microsoft.com/en-us/library/system.globalization.japanesecalendar(v=vs.110).aspx) and [JapaneseLunisolarCalendar](https://msdn.microsoft.com/en-us/library/system.globalization.japaneselunisolarcalendar(v=vs.110).aspx) and Microsoft's [Era Handling for the Japanese Calendar](https://docs.microsoft.com/en-us/windows/desktop/intl/era-handling-for-the-japanese-calendar)

## About This Project
The name Jikan comes from *jikan*, or more properly 時間 or じかん, the Japanese word for time and is pronounced jeeKAHN, or [d͡ʑikã̠ɴ] in IPA. This name was chosen because the first calendar conversion Jikan is set to implement is Georgian calendar dates to dates that use the Japanese era calendar scheme. While this is being used as a starting point, the plan is for Jikan to eventually be expanded to cover other calendar systems, which is the reason a word meaning time was chosen rather than one specifically related to Japanese era names.

Jikan is being written by a one-member team as a final project for an undergraduate introductory Rust course. I want to be upfront in saying I don't have a lot of experience in Rust or library creation in general. That said, I'm going to do my best to turn this into a fully functional library that complies with Rust standards and best practices. I also welcome feedback and collaboration in the form of direct contact and forking and pull requests. 

My background and contact information is as follows:
* Melissa Calawa
* [mccalawa@gmail.com](mailto:mccalawa@gmail.com)
* Anticipated to graduate with a BS in computer science December of 2018
* Post-bacc student with a BA in English
* Previous coding experience includes C, C++, C#, Java, Phython, Haskell, Visual Basic, POV-Ray, ASP.NET MVC, HTML, CSS, JavaScript, JQuery, and related technologies

## The Calendar Systems
When designing a library like this, there are always multiple things that should beconsidered. In this section, I will go over how some of the date systems work and what some of the various design considerations for that particular system are.

### The Japanese Era Calendar
The Japanese era calendar scheme refers to the system used for measuring the year in Japan. In much of East Asia, the years were numbered based on the current reigning ruler. Like other East Asian countries such as Korea and Vietnam, this pratice originated from a Chinese practice that was brought to other countries, though the exact scheme used by each country often varied based on the country's own rulers. Unlike most other countries that used a similar scheme, Japan still uses this system along with Georgian years.

A Japanese era name is known as *nengō* (alternatively romanized as *nengou*), which as written as 年号 in Japanese, or *gengō* (alternatively romanized as *gengou*), which is written as 元号 in Japanese. *Nengō* is written with the characters for year and name, whereas *gengō* is written with the characters for origin and name. *Nengō* are made up of two elements, the first being the name of the era itself, while the second is the number of years since the era began. *Nengō* always start at 1. There is no such thing as a year 0 in the Japanese era calendar scheme.

While specific *nengō* are sometimes are sometimes called periods rather than eras, period is also commonly used as a translation for the Japanese word *jidai* or 時代, which is a larger divisions of Japanese history where the change of period is marked by some sort of major event such as a war, change in government, or relocation of the capital. In modern Japan, *nengō* and *jidai* intentionally overlap, but prior to 1868, a *jidai* could be made up of multiple *nengō*. For this reason, Jikai will refer to *nengō* exclusively by the name era and use period to exclusively refer to *jidai*.

#### *Nengō* Prior to 1868
The use of *nengō* began in 645 with the creation of the Taika (or 大化) era. The Taika era was followed by the Hakuchi (or 白雉) era, which ended in 654. Hakuchi was followed by a gap in era naming that lasted until the beginning of the Shuchō (or 朱鳥, with the alternate romanization of Shuchou) era in June of 686. After the end of the Shuchō era in September of the same year, there was another gap that lasted until the beginning of the Taihō (or 大宝, with the alternate romanization of Taihou) era in March of 701. Era naming continued without interruption following the Taihō era to present day. 

During this period of Japanese history, there was not an exact rule that determined when era names changed. A single emperor or emperoress would often have multiple eras during their rule with era changes for inauspicious years, natural disasters, and other important events. The first day of a new *nengō* would start on a day of the current ruler's choosing and continue until the lunar new year, when the second year of that *nengō* would start.

Additionally, a lunisolar calendar (as opposed to a solar calendar like the Georgian calendar) based on the Chinese calendar was at use in Japan at the time. This calendar was known as the *onmyōreki* (alternatively romanized as *onmyoureki*), which is written as 陰陽暦 in Japanese and translates to yin-yang calendar, or the *taiintaiyōreki* (alternatively romanized as *taiintaiyoureki*), written as 太陰太陽暦 in Japanese and directly translating to lunisolar calendar. 

The *onmyōreki* was made up of 12 months with 30 or 29 days each. Months with 30 days were known as *dai no tsuki* or 大の月, which means long months, and months with 29 days were known as *shō no tsuki* (alternatively romanized as *shou no tsuki*) or 小の月, which means short months. (As a note, the phrase *dai no tsuki* is used to refer to months with 31 days and *shō no tsuki* is used for months with 30 days in modern Japanese.) There was not a set system for deciding whether a month would have 30 days or 29. Instead, this was decided on a yearly basis by the imperial court and calendars were used by average people to learn which months would have 30 days and which ones would have 29 in any given year.

Because of the use of 12 29-to-30-day months leads to a year that is roughly 354 days long rather than the 365.25 days that we now know a solar year is made up of, an intercalary month (*uruuzuki* or 閏月) was included in the calendar every few years in order to control seasonal shift. Like the non-standardized month lengths, there was not a set placement for the intercalary month. It was instead denoted in the calendar by the same number as the month prior to it with the kanji for intercalary proceeding it. (Japanese months are denoted by number rather than name. For example, the month of May is written as 五月 or *gogatsu*, meaning fifth month. So, in a year with an intercalary month that comes after May and before June, the intercalary month would be written as 閏五月 or *uruugogatsu*, meaning intercalary fifth month.)

#### *Nengō* from 1868 and Beyond
There were several variations in the exact calendar format used prior to 1868, but all of the calendars used were lunisolar calendars with fairly similar formats. It wasn't until the beginning of the Meiji (or 明治) era in 1868 that major changes to the system were implemented.

The first of these major changes was the adoption of a *issei-ichigen* (一世一元) or "one reign, one era name" system. This means that the era would only change upon the succession of the new emperor. The other major change was the adoption of the Georgian calendar. With the adoption of the Georgian calendar, the first year in a *nengō* would last from the date of succession to December 31st. 