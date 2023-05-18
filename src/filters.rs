use crate::input::Args;
use crate::parser::Issue;

/*
    Filters for the json input.
    Filter can be either on dates, labels or state.
*/

pub struct Filters {
    pub label_filter: LabelFilter,
}

pub struct LabelFilter {
    pub active: bool,
    pub pattern: String,
}

impl Filters {
    pub fn from_args(args: &Args) -> Filters {
        let label_filter = LabelFilter {
            active: args.label != "",
            pattern: String::from(&args.label),
        };

        Filters {
            label_filter: label_filter,
        }
    }

    pub fn is_filtered(&self, issue: &Issue) -> bool {
        if issue.is_labeled(&self.label_filter.pattern) || !self.label_filter.active {
            return true;
        }

        false
    }
}
