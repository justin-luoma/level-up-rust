use chrono::NaiveDate;

fn weeks_between(a: &str, b: &str) -> i32 {
    let a = NaiveDate::parse_from_str(a, "%Y-%m-%d").unwrap();
    let b = NaiveDate::parse_from_str(b, "%Y-%m-%d").unwrap();
    (b - a).num_weeks() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_day() {
        let n_weeks = weeks_between("1010-10-10", "1010-10-10");
        assert_eq!(n_weeks, 0);
    }

    #[test]
    fn one_week() {
        let n_weeks = weeks_between("1010-10-10", "1010-10-18");
        assert_eq!(n_weeks, 1);
    }

    #[test]
    fn past() {
        let n_weeks = weeks_between("1010-10-18", "1010-10-10");
        assert_eq!(n_weeks, -1);
    }
}
