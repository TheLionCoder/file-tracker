use clap::{Arg, ArgMatches};

pub(crate) fn parse_cli() -> ArgMatches {
    clap::Command::new("File Length Inventory")
        .version("0.1.0")
        .author("TheLionCoder")
        .about("Counts the number of lines in files in a directory")
        .arg(
            Arg::new("dir-path")
                .short('d')
                .long("dir")
                .required(true)
                .help("The path to the directory to be inventoried"),
        )
        .arg(
            Arg::new("header")
                .short('h')
                .long("header")
                .action(clap::ArgAction::SetTrue)
                .help("The maximum depth to search for files"),
        )
        .arg(
            Arg::new("max-depth")
                .short('m')
                .long("depth")
                .default_value("1")
                .help("The max depth of inventory to be."),
        )
        .get_matches()
}

#[cfg(test)]
mod tests {

    use super::*;
    use clap::Command;

    #[test]
    fn test_parse_cli_without_header() {
        let matches = Command::new("test")
            .arg(
                Arg::new("header")
                    .short('r')
                    .long("header")
                    .default_value("true"),
            )
            .try_get_matches_from(vec!["test", "-r", "false"])
            .unwrap();
        assert_eq!(
            matches
                .get_one::<String>("header")
                .map(String::as_str)
                .unwrap(),
            "false"
        );
    }

    #[test]
    fn test_parse_cli_with_header() {
        let matches = Command::new("test")
            .arg(
                Arg::new("header")
                    .short('r')
                    .long("header")
                    .action(clap::ArgAction::SetTrue),
            )
            .try_get_matches_from(vec!["test", "--header"])
            .unwrap();
        assert!(matches.get_flag("header"));
    }

    #[test]
    fn test_parse_cli_dir() {
        let matches = Command::new("test")
            .arg(Arg::new("dir_path").short('p').long("path"))
            .try_get_matches_from(["test", "--path", "./assets/data"])
            .unwrap();
        assert_eq!(
            matches.get_one::<String>("dir_path").unwrap(),
            "./assets/data"
        )
    }

    #[test]
    fn test_parse_cli_min_depth() {
        let matches = Command::new("test")
            .arg(Arg::new("max_depth").short('t').long("depth"))
            .try_get_matches_from(vec!["test", "-t", "2"])
            .unwrap();
        assert_eq!(matches.get_one::<String>("max_depth").unwrap(), "2")
    }
}
