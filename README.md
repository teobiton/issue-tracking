# Issue tracking

This command-line tool can be used to parse JSON files containg data from a GitHub repository issues and export it to csv.
The purpose is to retrieve the data easily, pre-process it with the tool and then only keep valuable information.
The information retrieved consists of an issue's ID, its creation date, last update date, state and title.

## Example

Command:

```bash
$ issue-tracking teobiton/issue-tracking --get --output=example.csv --state=open
```
Output in `example.csv`:

```
ID,Created at,Last update,State,Comment
9,2023-06-01,2023-06-01,open,Create `build.rs` for installation
1,2023-05-15,2023-05-15,open,Move from StructOpt to App
```

## Usage

The tool requires a JSON file containing GitHub issues, or a valid GitHub repository name.
A valid JSON file can be retrieved either with a plugin from a browser or by accessing GitHub API for a repository with `curl`.
This is the help message of the application:

```
issue-tracking 1.0.0
GitHub issues parser and exporter from JSON to csv

USAGE:
    issue-tracking [FLAGS] [OPTIONS] <json>

FLAGS:
    -g, --get             GET the JSON file from GitHub API.
    -h, --help            Prints help information
        --print-labels    Print all available labels in the repository.
    -V, --version         Prints version information

OPTIONS:
        --end-date <end-date>        Only consider issues updated before this date. Format: YYYY-MM-DD [default:
                                     (newest)]
    -l, --label <label>              Filter the issues based on a label. [default: (all)]
    -o, --output <output>            Specify a file to store the csv. [default: out.csv]
        --start-date <start-date>    Only consider issues updated after this date. Format: YYYY-MM-DD [default:
                                     (oldest)]
    -s, --state <state>              Only consider issues that have a particular state. [default: (any)]

ARGS:
    <json>    Required JSON file link (local or from GitHub API).
```

If you're looking for specific issues, you can specify a label to search for when parsing the JSON file.
You can also constraint the output between two dates, or only look for closed/opened issues.