use crate::date::Date;
use crate::input::Args;
use crate::parser::Issue;

/*
    Filters for the json input.
    Filter can be either on dates, labels or state.
*/

// Primary filters structure
pub struct Filters {
    pub label_filter: LabelFilter,
    pub state_filter: StateFilter,
    pub date_filter: DateFilter,
}

// Structure for labels
pub struct LabelFilter {
    pub active: bool,
    pub pattern: String,
}

impl LabelFilter {
    pub fn reject(&self, issue: &Issue) -> bool {
        // Returns true if the issue is rejected by label filter

        if !issue.is_labeled(&self.pattern) && self.active {
            return true;
        }

        false
    }
}

// Structure for state
pub struct StateFilter {
    pub active: bool,
    pub pattern: String,
}

impl StateFilter {
    pub fn reject(&self, issue: &Issue) -> bool {
        // Returns true if the issue is rejected by state filter

        if (issue.state != self.pattern) && self.active {
            return true;
        }

        false
    }
}

// Structure for state
pub struct DateFilter {
    pub start_active: bool,
    pub start_date: Date,
    pub end_active: bool,
    pub end_date: Date,
}

impl DateFilter {
    pub fn reject(&self, issue: &Issue) -> bool {
        // Returns true if the issue is out of bounds

        let issue_date: Date = Date::from_str(&issue.updated_at[..10]);

        if (self.start_date.compare(&issue_date) > 0) && self.start_active {
            return true;
        }

        if (self.end_date.compare(&issue_date) < 0) && self.end_active {
            return true;
        }

        false
    }
}

impl Filters {
    pub fn from_args(args: &Args) -> Filters {
        // Factory function that builds filters from user inputs
        let label_filter = LabelFilter {
            active: args.label != "(all)",
            pattern: String::from(&args.label),
        };

        let state_filter = StateFilter {
            active: args.state != "(any)",
            pattern: String::from(&args.state),
        };

        let date_filter = DateFilter {
            start_active: args.start_date != "(oldest)",
            start_date: Date::from_str(&args.start_date),
            end_active: args.end_date != "(newest)",
            end_date: Date::from_str(&args.end_date),
        };

        Filters {
            label_filter: label_filter,
            state_filter: state_filter,
            date_filter: date_filter,
        }
    }

    pub fn reject(&self, issue: &Issue) -> bool {
        // Returns true if the issue is rejected by the filters

        if self.label_filter.reject(issue) {
            return true;
        }

        if self.state_filter.reject(issue) {
            return true;
        }

        if self.date_filter.reject(issue) {
            return true;
        }

        false
    }
}
