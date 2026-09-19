// Command line argument parsing

use clap::{value_parser, Arg, ArgGroup, Command};

/// Volume step accepted by `-a` / `-m`, in percent.
const VOLUME_RANGE: std::ops::RangeInclusive<i64> = 1..=100;

/// What the program should do for the given command line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// No argument given: open the interactive TUI.
    Tui,

    /// `-a` / `-m` given: change the volume once, then exit.
    Change { steps: u16, increase: bool },
}

/// Build the command line interface.
fn command() -> Command {
    Command::new("ramixer")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A simple TUI volume mixer based on amixer")
        .group(
            ArgGroup::new("change")
                .args(["increase", "decrease"])
                .multiple(false),
        )
        .arg(
            Arg::new("increase")
                .short('a')
                .long("add")
                .value_name("VOLUME")
                .help("Increase the master volume by VOLUME percent, then exit")
                .value_parser(value_parser!(u16).range(VOLUME_RANGE)),
        )
        .arg(
            Arg::new("decrease")
                .short('m')
                .long("minus")
                .value_name("VOLUME")
                .help("Decrease the master volume by VOLUME percent, then exit")
                .value_parser(value_parser!(u16).range(VOLUME_RANGE)),
        )
}

/// Parse the process arguments into an [`Action`].
///
/// `--help` and `--version` are handled by clap itself, which prints the
/// information and exits the process.
pub fn parse() -> Action {
    action_from(command().get_matches())
}

/// Turn parsed arguments into an [`Action`].
fn action_from(matches: clap::ArgMatches) -> Action {
    if let Some(steps) = matches.get_one::<u16>("increase") {
        Action::Change {
            steps: *steps,
            increase: true,
        }
    } else if let Some(steps) = matches.get_one::<u16>("decrease") {
        Action::Change {
            steps: *steps,
            increase: false,
        }
    } else {
        Action::Tui
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn action(args: &[&str]) -> Action {
        action_from(command().try_get_matches_from(args).unwrap())
    }

    fn rejected(args: &[&str]) -> bool {
        command().try_get_matches_from(args).is_err()
    }

    #[test]
    fn no_argument_opens_tui() {
        assert_eq!(action(&["ramixer"]), Action::Tui);
    }

    #[test]
    fn add_flag_increases_volume() {
        let expected = Action::Change {
            steps: 10,
            increase: true,
        };

        assert_eq!(action(&["ramixer", "-a", "10"]), expected);
        assert_eq!(action(&["ramixer", "--add", "10"]), expected);
    }

    #[test]
    fn minus_flag_decreases_volume() {
        let expected = Action::Change {
            steps: 10,
            increase: false,
        };

        assert_eq!(action(&["ramixer", "-m", "10"]), expected);
        assert_eq!(action(&["ramixer", "--minus", "10"]), expected);
    }

    #[test]
    fn volume_must_be_within_one_to_hundred() {
        assert!(rejected(&["ramixer", "-a", "0"]));
        assert!(rejected(&["ramixer", "-a", "101"]));
        assert!(rejected(&["ramixer", "-m", "abc"]));
    }

    #[test]
    fn add_and_minus_are_mutually_exclusive() {
        assert!(rejected(&["ramixer", "-a", "5", "-m", "5"]));
    }
}
