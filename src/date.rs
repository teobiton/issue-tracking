pub struct Date {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

impl Date {
    pub fn from_str(date_str: &str) -> Date {
        // Build a date structure from a string

        if date_str == "(oldest)" || date_str == "(newest)" {
            return Date {
                year: 0,
                month: 0,
                day: 0,
            };
        }
        println!(
            ",,{},,{},,{},,{},,",
            &date_str,
            &date_str[..4],
            &date_str[5..7],
            &date_str[8..10]
        );

        Date {
            year: date_str[..4].parse().unwrap(),
            month: date_str[5..7].parse().unwrap(),
            day: date_str[8..10].parse().unwrap(),
        }
    }

    pub fn before(&self, date: &Date) -> bool {
        // Returns true if the date structure is older than the input
        if self.year < date.year {
            return true;
        }

        if self.year == date.year {
            if self.month < date.month {
                return true;
            }

            if self.month == date.month {
                if self.day <= date.day {
                    return true;
                }
            }
        }

        false
    }
}
