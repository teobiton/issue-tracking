pub struct Date {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

impl Date {
    pub fn date_str(date_str: &str) -> Date {
        // Build a date structure from a string

        if date_str == "(oldest)" || date_str == "(newest)" {
            return Date {
                year: 0,
                month: 0,
                day: 0,
            };
        }

        Date {
            year: date_str[..4].parse().unwrap(),
            month: date_str[5..7].parse().unwrap(),
            day: date_str[8..10].parse().unwrap(),
        }
    }

    pub fn compare(&self, date: &Date) -> i32 {
        // Returns true if the date structure is older than the input

        let mut result: i32 = self.year - date.year;

        if result != 0 {
            return result;
        }

        result = self.month - date.month;

        if result != 0 {
            return result;
        }

        result = self.day - date.day;

        if result != 0 {
            return result;
        }

        result
    }
}
