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
    pub fn is_filtered(&self, issue: &Issue) -> bool {
        // Returns true if the issue is accepted by label filter

        if issue.is_labeled(&self.pattern) && self.active {
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
    pub fn is_filtered(&self, issue: &Issue) -> bool {
        // Returns true if the issue is accepted by state filter

        if (issue.state == self.pattern) && self.active {
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
    pub fn is_filtered(&self, issue: &Issue) -> bool {
        // Returns true if the issue is between  by state filter

        let issue_date: Date = Date::from_str(&issue.created_at[..10]);

        if (self.start_date.before(&issue_date)) && self.start_active {
            return true;
        }

        if !(self.end_date.before(&issue_date)) && self.end_active {
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
            start_active: args.from_date != "(oldest)",
            start_date: Date::from_str(&args.from_date),
            end_active: args.from_date != "(newest)",
            end_date: Date::from_str(&args.until_date),
        };

        Filters {
            label_filter: label_filter,
            state_filter: state_filter,
            date_filter: date_filter,
        }
    }

    pub fn is_filtered(&self, issue: &Issue) -> bool {
        // Returns true if the issue is accepted by the filters

        if self.label_filter.is_filtered(issue) {
            return true;
        }

        if self.state_filter.is_filtered(issue) {
            return true;
        }

        if self.date_filter.is_filtered(issue) {
            return true;
        }

        false
    }
}
