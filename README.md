# File Tracker

This project is designed to list the plain text (`.txt`)  
and csv (`.csv`) files in a directory and create an **_inventory_**
in a Excel file, which will be saved in the folder where the files are located.

## Installation

1. Clone the repository:

```sh
git clone git@github.com:TheLionCoder/file-tracker.git
cd file-tracker
```

### Usage

To run the program , use the following command:

_Get Help_:

```sh
cargo run  -- --help
```

#### _Arguments_

- `-d, --dir <dir-path> The path to the directory to be inventoried`
- `-h, --header        Indicates whether the files have a header`
- `-m, --depth <max-depth> The max depth to search for files. [default 1]`

### Example

To inventory files in **_two_** subdirectories where the file have **headers** .

```sh
cargo run -- -d data/ -h -depth 2
```
