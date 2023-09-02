use chrono::NaiveDate;

/// Parses a string that represents a date. When a date
/// is unable to be determined, return `None`.
fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
    if let Ok(date) = NaiveDate::parse_from_str(text, "%Y-%m-%d") {
        Some(date)
    } else if let Ok(date) = NaiveDate::parse_from_str(text, "%Y/%b/%d") {
        Some(date)
    } else if let Ok(date) = NaiveDate::parse_from_str(text, "%d.%b.%Y") {
        Some(date)
    } else if let Ok(date) = NaiveDate::parse_from_str(text, "%b.%d.%Y") {
        Some(date)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ymd_hyphen() {
        assert_eq!(flexible_date_parse("2010-12-11"), Some(NaiveDate::from_ymd(2010, 12, 11)))
    }

    #[test]
    fn ymd_slash() {
        assert_eq!(flexible_date_parse("1999/Mar/02"), Some(NaiveDate::from_ymd(1999, 3, 2)))
    }

    #[test]
    fn dmy_dot() {
        assert_eq!(flexible_date_parse("01.Mar.2021"), Some(NaiveDate::from_ymd(2021, 3, 1)))
    }

    #[test]
    fn mdy_dot() {
        assert_eq!(flexible_date_parse("Apr.05.2021"), Some(NaiveDate::from_ymd(2021, 4, 5)))
    }

    #[test]
    fn invalid() {
        assert_eq!(flexible_date_parse("not a date"), None)
    }
}
