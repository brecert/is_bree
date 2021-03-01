use datetime::{DatePiece, Month, Weekday, Year};

pub fn is_bree<D: DatePiece>(date: D) -> bool {
    let year = Year(date.year());
    let day = date.day();

    date.weekday() == Weekday::Friday
        && 0 <= day - 7
        && 0 >= day - 14
        && !(year.is_leap_year() && date.month() == Month::March)
        && !(date.year() == 2023 && date.month() == Month::July)
}

#[test]
fn test() {
    use datetime::LocalDate;

    let tests = [
        (
            "not a friday",
            LocalDate::ymd(2020, Month::November, 25),
            false,
        ),
        (
            "first friday of month",
            LocalDate::ymd(2020, Month::October, 2),
            false,
        ),
        (
            "second friday of month",
            LocalDate::ymd(2020, Month::June, 12),
            true,
        ),
        (
            "third friday of month",
            LocalDate::ymd(2020, Month::November, 20),
            false,
        ),
        (
            "fourth friday of month",
            LocalDate::ymd(2020, Month::January, 24),
            false,
        ),
        (
            "second friday, march, not leap year (1)",
            LocalDate::ymd(2019, Month::March, 8),
            true,
        ),
        (
            "second friday, march, leap year (1)",
            LocalDate::ymd(2020, Month::March, 13),
            false,
        ),
        (
            "second friday, march, not leap year (2)",
            LocalDate::ymd(1900, Month::March, 9),
            true,
        ),
        (
            "second friday, march, leap year (2)",
            LocalDate::ymd(2000, Month::March, 10),
            false,
        ),
        (
            "any other second friday, leap year",
            LocalDate::ymd(2000, Month::May, 12),
            true,
        ),
        (
            "july 2023 friday",
            LocalDate::ymd(2023, Month::July, 7),
            false,
        ),
        ("march 2023", LocalDate::ymd(2023, Month::March, 10), true),
        (
            "october 2022",
            LocalDate::ymd(2022, Month::October, 14),
            true,
        ),
    ];

    for (name, date, should_be_bree) in &tests {
        let am_bree = is_bree(date.unwrap());
        assert!(
            am_bree == *should_be_bree,
            "{} expected: {} actual: {}",
            name,
            should_be_bree,
            am_bree
        )
    }
}
