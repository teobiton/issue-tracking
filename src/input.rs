use structopt::StructOpt;

/*
    Defines the inputs supported by the application, using StructOpt.
*/

#[derive(StructOpt)]
#[structopt(
    name = "GitHub issues parser",
    about = "GitHub issues parser and exporter from JSON to csv"
)]
pub struct Args {
    /// --output=file
    #[structopt(
        long = "--output",
        short = "-o",
        default_value = "",
        help = "Specify a file to store the csv."
    )]
    pub output: String,

    /// --label=label
    #[structopt(
        long = "--label",
        short = "-l",
        default_value = "",
        help = "Filter the issues based on a label."
    )]
    pub label: String,

    /// --from-date=date
    #[structopt(
        long = "--from-date",
        default_value = "",
        help = "Only consider issues updated after this date. Format: YYYY-MM-DD"
    )]
    pub from_date: String,

    /// --until-date=date
    #[structopt(
        long = "--until-date",
        default_value = "",
        help = "Only consider issues updated before this date. Format: YYYY-MM-DD"
    )]
    pub until_date: String,

    /// --status=status
    #[structopt(
        long = "--status",
        short = "-s",
        default_value = "",
        help = "Only consider issues that have a particular status."
    )]
    pub status: String,

    /// Positional argument
    #[structopt(help = "Required JSON file.")]
    pub json: String,
}
