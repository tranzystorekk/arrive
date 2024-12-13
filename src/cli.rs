use clap::{ArgGroup, Args, Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser)]
#[command(version)]
pub enum Cli {
    /// Manage AOC session token
    ///
    /// You can find the token for your AOC session (opened in current tab)
    /// by opening your browser's developer tools,
    /// navigating to a "Data" section and looking for a cookie named "session"
    #[command(subcommand)]
    Token(TokenCmd),

    /// Print selected input file
    Input(InputArgs),

    /// Submit solution for pending task
    Submit {
        /// Solution string to be submitted.
        ///
        /// If omitted, read from STDIN instead.
        solution: Option<String>,
    },

    /// Select advent day
    #[command(group(ArgGroup::new("selection").multiple(true).required(true)))]
    Select {
        /// Year to select
        ///
        /// If supplied, requires --day as well. Otherwise, previously selected year is assumed.
        #[arg(group = "selection")]
        #[arg(short, long, value_parser = clap::value_parser!(u32).range(2015..), requires = "select_day")]
        year: Option<u32>,

        /// Day to select
        #[arg(group = "selection")]
        #[arg(id = "select_day", short = 'd', long = "day", value_name = "DAY", value_parser = clap::value_parser!(u32).range(1..=25))]
        day: Option<u32>,
    },

    /// Show current selection, solution status etc.
    Status,

    /// Print shell completion script
    Completion {
        /// Shell to print completion for
        shell: Shell,
    },
}

#[derive(Subcommand)]
pub enum TokenCmd {
    /// Set session token to be used
    Set {
        /// Session token to be stored
        token: String,
    },

    /// Print currently stored token
    Show,
}

#[derive(Args)]
pub struct InputArgs {
    /// Force fetch over the web and cache overwrite
    #[arg(short, long)]
    pub force: bool,

    /// Year to fetch input for
    ///
    /// If supplied, requires --day as well. Otherwise, currently selected year is assumed.
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(2015..), requires = "input_day")]
    pub year: Option<u32>,

    /// Day to fetch input for
    #[arg(id = "input_day", short = 'd', long = "day", value_name = "DAY", value_parser = clap::value_parser!(u32).range(1..25))]
    pub day: Option<u32>,
}
