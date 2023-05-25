# Issue parser

This command-line tool can be used to parse JSON files containg data from a GitHub repository issues and export it to csv.
The purpose is to retrieve the data easily, pre-process it with the tool and then only keep valuable information.
The information retrieved consists of an issue's ID, its creation date, last update date, status and title.

Example output, with issues from this repository:

```
ID,Created at,Last update,Status,Comment
1,2023-05-15,2023-05-15,open,Move from StructOpt to App
2,2023-05-16,2023-05-16,closed,Improve error management
3,2023-05-17,2023-05-17,open,Introduce filtering capabilities
```

## Usage

The tool requires a JSON file containing GitHub issues.
This can be retrieved either with a plugin from a browser by accessing GitHub API for a specify repository with `curl`.
This is the help message of the application:

```
issue-parser 1.0.0
GitHub issues parser and exporter from JSON to csv

USAGE:
    issue-parser [FLAGS] [OPTIONS] <json>

FLAGS:
    -h, --help            Prints help information
        --print-labels    Print all available labels in the repository.
    -V, --version         Prints version information

OPTIONS:
    -l, --label <label>      Filter the issues based on a label. [default: (all)]
    -o, --output <output>    Specify a file to store the csv. [default: out.csv]

ARGS:
    <json>    Required JSON file.
```

If you're looking for specific issues, you can specify a label to search for when parsing the JSON file.