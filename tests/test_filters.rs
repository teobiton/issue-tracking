use issue_parser::filters::Filters;
use issue_parser::filters::LabelFilter;

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
        print_labels: false,
        json: String::from(""),
    };

    let filter: Filters = Filters::from_args(&args);

    assert_eq!(filter.label_filter.active, true);
    assert_eq!(filter.label_filter.pattern, "type:feature");

    Ok(())
}

#[test]
fn test_build_unused_filters() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args {
        output: String::from(""),
        label: String::from("(all)"),
        print_labels: false,
        json: String::from(""),
    };

    let filter: Filters = Filters::from_args(&args);

    assert_eq!(filter.label_filter.active, false);
    assert_eq!(filter.label_filter.pattern, "(all)");

    Ok(())
}

#[test]
fn test_label_filtering() -> Result<(), Box<dyn std::error::Error>> {
    let filter: Filters = Filters {
        label_filter: LabelFilter {
            active: true,
            pattern: String::from("type:feature"),
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
        created_at: String::from(""),
        updated_at: String::from(""),
        closed_at: Some(String::from("")),
    };

    let issue_b: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: labels_b,
        state: String::from(""),
        created_at: String::from(""),
        updated_at: String::from(""),
        closed_at: Some(String::from("")),
    };

    let issue_c: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: labels_c,
        state: String::from(""),
        created_at: String::from(""),
        updated_at: String::from(""),
        closed_at: Some(String::from("")),
    };

    let issue_d: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: Vec::new(),
        state: String::from(""),
        created_at: String::from(""),
        updated_at: String::from(""),
        closed_at: Some(String::from("")),
    };

    let issues: [Issue; 4] = [issue_a, issue_b, issue_c, issue_d];

    let expected: [bool; 4] = [true, true, false, false];

    for n in 0..3 {
        assert_eq!(filter.is_filtered(&issues[n]), expected[n]);
    }

    Ok(())
}