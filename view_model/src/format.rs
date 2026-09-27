use std::time::SystemTime;

use chrono::{DateTime, Datelike, Local, NaiveDate};

const KB: u64 = 1024;
const MB: u64 = KB * KB;
const MONTHS: [&str; 12] = [
    "янв.",
    "февр.",
    "мар.",
    "апр.",
    "мая",
    "июн.",
    "июл.",
    "авг.",
    "сент.",
    "окт.",
    "нояб.",
    "дек.",
];

pub fn format_size(bytes: u64) -> String {
    if bytes < KB {
        return format!("{bytes} Б");
    }
    let kb = bytes.div_ceil(KB);
    if kb < KB {
        format!("{kb} КБ")
    } else {
        format!("{:.1} МБ", bytes as f64 / MB as f64)
    }
}

pub fn format_date(time: SystemTime) -> String {
    format_naive_date(DateTime::<Local>::from(time).date_naive())
}
fn format_naive_date(date: NaiveDate) -> String {
    let month = MONTHS[date.month0() as usize];
    format!("{} {month} {}", date.day(), date.year())
}

pub fn format_length_km(km: f64) -> String {
    format!("{km:.1} км")
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use rstest::rstest;

    use crate::format::{MB, format_naive_date, format_size};

    #[rstest]
    #[case::bytes(1023, "1023 Б")]
    #[case::exact_kb(1024, "1 КБ")]
    #[case::kb_rounds_up(1025, "2 КБ")]
    #[case::rounds_up_into_mb(MB - 1, "1.0 МБ")]
    #[case::exact_mb(MB, "1.0 МБ")]
    fn format_size_cases(#[case] bytes: u64, #[case] expected: &str) {
        assert_eq!(format_size(bytes), expected);
    }

    #[rstest]
    #[case::abbreviated(2026, 6, 16, "16 июн. 2026")]
    #[case::may_is_not_shortened(2026, 5, 28, "28 мая 2026")]
    #[case::single_digit_day(2026, 1, 3, "3 янв. 2026")]
    #[case::december(2025, 12, 31, "31 дек. 2025")]
    fn date(#[case] year: i32, #[case] month: u32, #[case] day: u32, #[case] expected: &str) {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        assert_eq!(format_naive_date(date), expected);
    }
}
