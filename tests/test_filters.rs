use issue_parser::filters::DateFilter;
use issue_parser::filters::Filters;
use issue_parser::filters::LabelFilter;
use issue_parser::filters::StateFilter;

use issue_parser::date::Date;
use issue_parser::input::Args;
use issue_parser::parser::Issue;
use issue_parser::parser::Label;

/*
   Tests for filters.
*/

#[test]
fn test_build_filters() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args {
        output: String::from(""),
        label: String::from("type:feature"),
        state: String::from("opened"),
        print_labels: false,
        from_date: String::from("(oldest)"),
        until_date: String::from("(newest)"),
        json: String::from(""),
    };

    let filter: Filters = Filters::from_args(&args);

    assert_eq!(filter.label_filter.active, true);
    assert_eq!(filter.label_filter.pattern, "type:feature");

    assert_eq!(filter.state_filter.active, true);
    assert_eq!(filter.state_filter.pattern, "opened");

    Ok(())
}

#[test]
fn test_build_unused_filters() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args {
        output: String::from(""),
        label: String::from("(all)"),
        state: String::from("(any)"),
        print_labels: false,
        from_date: String::from("(oldest)"),
        until_date: String::from("(newest)"),
        json: String::from(""),
    };

    let filter: Filters = Filters::from_args(&args);

    assert_eq!(filter.label_filter.active, false);
    assert_eq!(filter.label_filter.pattern, "(all)");

    assert_eq!(filter.state_filter.active, false);
    assert_eq!(filter.state_filter.pattern, "(any)");

    Ok(())
}

#[test]
fn test_label_filtering() -> Result<(), Box<dyn std::error::Error>> {
    let start_date: Date = Date {
        year: 2023,
        month: 9,
        day: 1,
    };

    let end_date: Date = Date {
        year: 2023,
        month: 9,
        day: 1,
    };

    let filter: Filters = Filters {
        label_filter: LabelFilter {
            active: true,
            pattern: String::from("type:feature"),
        },

        state_filter: StateFilter {
            active: false,
            pattern: String::from(""),
        },

        date_filter: DateFilter {
            start_active: false,
            start_date: start_date,
            end_active: false,
            end_date: end_date,
        },
    };

    let labels_a: Vec<Label> = vec![Label {
        name: String::from("type:feature"),
    }];

    let labels_b: Vec<Label> = vec![
        Label {
            name: String::from("rtl"),
        },
        Label {
            name: String::from("type:feature"),
        },
    ];

    let labels_c: Vec<Label> = vec![
        Label {
            name: String::from("others"),
        },
        Label {
            name: String::from("rtl"),
        },
    ];

    let issue_a: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: labels_a,
        state: String::from(""),
        created_at: String::from("2023-04-12"),
        updated_at: String::from("2023-04-12"),
        closed_at: Some(String::from("")),
    };

    let issue_b: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: labels_b,
        state: String::from(""),
        created_at: String::from("2023-04-12"),
        updated_at: String::from("2023-04-12"),
        closed_at: Some(String::from("")),
    };

    let issue_c: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: labels_c,
        state: String::from(""),
        created_at: String::from("2023-04-12"),
        updated_at: String::from("2023-04-12"),
        closed_at: Some(String::from("")),
    };

    let issue_d: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: Vec::new(),
        state: String::from(""),
        created_at: String::from("2023-04-12"),
        updated_at: String::from("2023-04-12"),
        closed_at: Some(String::from("")),
    };

    let issues: [Issue; 4] = [issue_a, issue_b, issue_c, issue_d];

    let expected: [bool; 4] = [true, true, false, false];

    for n in 0..3 {
        assert_eq!(filter.is_filtered(&issues[n]), expected[n]);
    }

    Ok(())
}
