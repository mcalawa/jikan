// This is part of Jikan
// See README.md for details

use chrono::prelude::*;

#[derive(Clone)]
pub struct Era {
    kanji: String,
    revised_hepburn: String,
    traditional_hepburn: String,
    nihon: String,
    kunrei: String,
    transliterated: String,
    start_month: enum,
    start_year: enum,
    end_month: enum,
    end_year: enum,
}