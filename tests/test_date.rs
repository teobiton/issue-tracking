use issue_parser::date::Date;

#[test]
fn test_build_date() -> Result<(), Box<dyn std::error::Error>> {
    let dates_str: [&str; 5] = [
        "1908-12-31",
        "2007-11-20",
        "0875-07-12",
        "0021-01-05",
        "0001-10-01",
    ];

    let dates_result: [Date; 5] = [
        Date {
            year: 1908,
            month: 12,
            day: 31,
        },
        Date {
            year: 2007,
            month: 11,
            day: 20,
        },
        Date {
            year: 875,
            month: 7,
            day: 12,
        },
        Date {
            year: 21,
            month: 1,
            day: 5,
        },
        Date {
            year: 1,
            month: 10,
            day: 1,
        },
    ];

    let mut temp_date: Date;
    for i in 0..5 {
        temp_date = Date::from_str(dates_str[i]);
        assert_eq!(temp_date.year, dates_result[i].year);
        assert_eq!(temp_date.month, dates_result[i].month);
        assert_eq!(temp_date.day, dates_result[i].day);
    }

    Ok(())
}

#[test]
fn test_compare_date() -> Result<(), Box<dyn std::error::Error>> {
    let example_date: Date = Date {
        year: 2023,
        month: 6,
        day: 18,
    };

    let dates_sample: [Date; 7] = [
        Date {
            year: 2023,
            month: 6,
            day: 17,
        },
        Date {
            year: 2023,
            month: 6,
            day: 18,
        },
        Date {
            year: 2023,
            month: 6,
            day: 19,
        },
        Date {
            year: 2023,
            month: 5,
            day: 19,
        },
        Date {
            year: 2023,
            month: 7,
            day: 19,
        },
        Date {
            year: 2022,
            month: 7,
            day: 19,
        },
        Date {
            year: 2024,
            month: 7,
            day: 19,
        },
    ];

    let expected: [bool; 7] = [false, true, true, false, true, false, true];

    for i in 0..5 {
        assert_eq!(example_date.before(&dates_sample[i]), expected[i]);
    }

    Ok(())
}
