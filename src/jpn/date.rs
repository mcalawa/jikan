use jpn::era::EraYear;

pub struct Date {
    day: u32,
    month: u32,
    year: EraYear,
}

impl Date {
    pub fn new(year: EraYear, month: u32, day: u32) -> Self {
        let date = Date {
            year: year,
            month: month,
            day: day,
        };
        date
    }
}