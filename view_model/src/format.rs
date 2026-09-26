use std::time::SystemTime;

use chrono::{DateTime, Local};

const KB: u64 = 1024;
const MB: u64 = KB * KB;

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
    DateTime::<Local>::from(time).format("%d.%m.%Y").to_string()
}

pub fn format_length_km(km: f64) -> String {
    format!("{km:.1} км")
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::format::{MB, format_size};

    #[rstest]
    #[case::bytes(1023, "1023 Б")]
    #[case::exact_kb(1024, "1 КБ")]
    #[case::kb_rounds_up(1025, "2 КБ")]
    #[case::rounds_up_into_mb(MB - 1, "1.0 МБ")]
    #[case::exact_mb(MB, "1.0 МБ")]
    fn format_size_cases(#[case] bytes: u64, #[case] expected: &str) {
        assert_eq!(format_size(bytes), expected);
    }
}
