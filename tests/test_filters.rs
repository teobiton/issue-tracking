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
        start_date: String::from("(oldest)"),
        end_date: String::from("(newest)"),
        json: String::from(""),
        get: false,
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
        start_date: String::from("(oldest)"),
        end_date: String::from("(newest)"),
        json: String::from(""),
        get: false,
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
        created_at: String::from("2020-06-15"),
        updated_at: String::from("2020-06-15"),
        closed_at: Some(String::from("")),
    };

    let issue_b: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: labels_b,
        state: String::from(""),
        created_at: String::from("2020-06-15"),
        updated_at: String::from("2020-06-15"),
        closed_at: Some(String::from("")),
    };

    let issue_c: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: labels_c,
        state: String::from(""),
        created_at: String::from("2020-06-15"),
        updated_at: String::from("2020-06-15"),
        closed_at: Some(String::from("")),
    };

    let issue_d: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: Vec::new(),
        state: String::from(""),
        created_at: String::from("2020-06-15"),
        updated_at: String::from("2020-06-15"),
        closed_at: Some(String::from("")),
    };

    let issues: [Issue; 4] = [issue_a, issue_b, issue_c, issue_d];

    let expected: [bool; 4] = [false, false, true, true];

    for n in 0..4 {
        assert_eq!(filter.reject(&issues[n]), expected[n]);
    }

    Ok(())
}

#[test]
fn test_start_date_filtering() -> Result<(), Box<dyn std::error::Error>> {
    let start_date: Date = Date {
        year: 2020,
        month: 6,
        day: 15,
    };

    let end_date: Date = Date {
        year: 2023,
        month: 8,
        day: 12,
    };

    let filter: Filters = Filters {
        label_filter: LabelFilter {
            active: false,
            pattern: String::from(""),
        },

        state_filter: StateFilter {
            active: false,
            pattern: String::from(""),
        },

        date_filter: DateFilter {
            start_active: true,
            start_date: start_date,
            end_active: false,
            end_date: end_date,
        },
    };

    let issue_a: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: Vec::new(),
        state: String::from(""),
        created_at: String::from("2020-06-16"),
        updated_at: String::from("2020-06-16"),
        closed_at: Some(String::from("")),
    };

    let issue_b: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: Vec::new(),
        state: String::from(""),
        created_at: String::from("2022-06-15"),
        updated_at: String::from("2022-06-15"),
        closed_at: Some(String::from("")),
    };

    let issue_c: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: Vec::new(),
        state: String::from(""),
        created_at: String::from("2020-06-11"),
        updated_at: String::from("2020-06-11"),
        closed_at: Some(String::from("")),
    };

    let issue_d: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: Vec::new(),
        state: String::from(""),
        created_at: String::from("2012-08-14"),
        updated_at: String::from("2012-08-14"),
        closed_at: Some(String::from("")),
    };

    let issues: [Issue; 4] = [issue_a, issue_b, issue_c, issue_d];

    let expected: [bool; 4] = [false, false, true, true];

    for n in 0..4 {
        println!("{} == {}", filter.reject(&issues[n]), expected[n]);
        assert_eq!(filter.reject(&issues[n]), expected[n]);
    }

    Ok(())
}

#[test]
fn test_end_date_filtering() -> Result<(), Box<dyn std::error::Error>> {
    let start_date: Date = Date {
        year: 2020,
        month: 6,
        day: 15,
    };

    let end_date: Date = Date {
        year: 2023,
        month: 8,
        day: 12,
    };

    let filter: Filters = Filters {
        label_filter: LabelFilter {
            active: false,
            pattern: String::from(""),
        },

        state_filter: StateFilter {
            active: false,
            pattern: String::from(""),
        },

        date_filter: DateFilter {
            start_active: true,
            start_date: start_date,
            end_active: true,
            end_date: end_date,
        },
    };

    let issue_a: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: Vec::new(),
        state: String::from(""),
        created_at: String::from("2020-06-16"),
        updated_at: String::from("2020-06-16"),
        closed_at: Some(String::from("")),
    };

    let issue_b: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: Vec::new(),
        state: String::from(""),
        created_at: String::from("2022-06-15"),
        updated_at: String::from("2022-06-15"),
        closed_at: Some(String::from("")),
    };

    let issue_c: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: Vec::new(),
        state: String::from(""),
        created_at: String::from("2024-06-15"),
        updated_at: String::from("2024-06-15"),
        closed_at: Some(String::from("")),
    };

    let issue_d: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: Vec::new(),
        state: String::from(""),
        created_at: String::from("2023-08-14"),
        updated_at: String::from("2023-08-14"),
        closed_at: Some(String::from("")),
    };

    let issues: [Issue; 4] = [issue_a, issue_b, issue_c, issue_d];

    let expected: [bool; 4] = [false, false, true, true];

    for n in 0..4 {
        println!("{} == {}", filter.reject(&issues[n]), expected[n]);
        assert_eq!(filter.reject(&issues[n]), expected[n]);
    }

    Ok(())
}

#[test]
fn test_inbetween_date_filtering() -> Result<(), Box<dyn std::error::Error>> {
    let start_date: Date = Date {
        year: 2020,
        month: 6,
        day: 15,
    };

    let end_date: Date = Date {
        year: 2023,
        month: 8,
        day: 12,
    };

    let filter: Filters = Filters {
        label_filter: LabelFilter {
            active: false,
            pattern: String::from(""),
        },

        state_filter: StateFilter {
            active: false,
            pattern: String::from(""),
        },

        date_filter: DateFilter {
            start_active: true,
            start_date: start_date,
            end_active: true,
            end_date: end_date,
        },
    };

    let issue_a: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: Vec::new(),
        state: String::from(""),
        created_at: String::from("2020-06-16"),
        updated_at: String::from("2020-06-16"),
        closed_at: Some(String::from("")),
    };

    let issue_b: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: Vec::new(),
        state: String::from(""),
        created_at: String::from("2022-06-15"),
        updated_at: String::from("2022-06-15"),
        closed_at: Some(String::from("")),
    };

    let issue_c: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: Vec::new(),
        state: String::from(""),
        created_at: String::from("2024-06-15"),
        updated_at: String::from("2024-06-15"),
        closed_at: Some(String::from("")),
    };

    let issue_d: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: Vec::new(),
        state: String::from(""),
        created_at: String::from("2023-08-14"),
        updated_at: String::from("2023-08-14"),
        closed_at: Some(String::from("")),
    };

    let issues: [Issue; 4] = [issue_a, issue_b, issue_c, issue_d];

    let expected: [bool; 4] = [false, false, true, true];

    for n in 0..4 {
        println!("{} == {}", filter.reject(&issues[n]), expected[n]);
        assert_eq!(filter.reject(&issues[n]), expected[n]);
    }

    Ok(())
}

#[test]
fn test_state_filtering() -> Result<(), Box<dyn std::error::Error>> {
    let start_date: Date = Date {
        year: 2020,
        month: 6,
        day: 15,
    };

    let end_date: Date = Date {
        year: 2023,
        month: 8,
        day: 12,
    };

    let filter: Filters = Filters {
        label_filter: LabelFilter {
            active: false,
            pattern: String::from(""),
        },

        state_filter: StateFilter {
            active: true,
            pattern: String::from("opened"),
        },

        date_filter: DateFilter {
            start_active: false,
            start_date: start_date,
            end_active: false,
            end_date: end_date,
        },
    };

    let issue_a: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: Vec::new(),
        state: String::from("opened"),
        created_at: String::from("2020-06-16"),
        updated_at: String::from("2020-06-16"),
        closed_at: Some(String::from("")),
    };

    let issue_b: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: Vec::new(),
        state: String::from("opened"),
        created_at: String::from("2022-06-15"),
        updated_at: String::from("2022-06-15"),
        closed_at: Some(String::from("")),
    };

    let issue_c: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: Vec::new(),
        state: String::from("closed"),
        created_at: String::from("2024-06-15"),
        updated_at: String::from("2024-06-15"),
        closed_at: Some(String::from("")),
    };

    let issue_d: Issue = Issue {
        title: String::from(""),
        number: 1,
        labels: Vec::new(),
        state: String::from("closed"),
        created_at: String::from("2023-08-14"),
        updated_at: String::from("2023-08-14"),
        closed_at: Some(String::from("")),
    };

    let issues: [Issue; 4] = [issue_a, issue_b, issue_c, issue_d];

    let expected: [bool; 4] = [false, false, true, true];

    for n in 0..4 {
        println!("{} == {}", filter.reject(&issues[n]), expected[n]);
        assert_eq!(filter.reject(&issues[n]), expected[n]);
    }

    Ok(())
}
