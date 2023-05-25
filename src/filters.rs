use crate::input::Args;
use crate::parser::Issue;

/*
    Filters for the json input.
    Filter can be either on dates, labels or state.
*/

// Primary filters structure
pub struct Filters {
    pub label_filter: LabelFilter,
}

// Structure for labels
pub struct LabelFilter {
    pub active: bool,
    pub pattern: String,
}

impl Filters {
    pub fn from_args(args: &Args) -> Filters {
        // Factory function that builds filters from user inputs
        let label_filter = LabelFilter {
            active: args.label != "(all)",
            pattern: String::from(&args.label),
        };

        Filters {
            label_filter: label_filter,
        }
    }

    pub fn is_filtered(&self, issue: &Issue) -> bool {
        // Returns true if the issue is accepted by the filters

        if issue.is_labeled(&self.label_filter.pattern) || !self.label_filter.active {
            return true;
        }

        false
    }
}
