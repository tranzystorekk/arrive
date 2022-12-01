mod cli;
mod paths;
mod state;
mod web;

use cli::{Cli, InputArgs, TokenCmd};
use state::State;

use clap::Parser;

fn main() {
    let cli = Cli::parse();

    if let Cli::Completion { shell } = cli {
        cli::generate_completion(shell);
        return;
    }

    let mut state = State::load();

    match cli {
        Cli::Token(TokenCmd::Set { token }) => state.session_token = Some(token),
        Cli::Token(TokenCmd::Show) => {
            do_token_show_cmd(&state);
            return;
        }
        Cli::Input(_) => todo!(),
        Cli::Submit { solution } => todo!(),
        Cli::Select { year, day } => {
            state.year = year;
            state.day = day;
        }
        Cli::Status => {
            state.print_status();
            return;
        }
        Cli::Completion { .. } => unreachable!(),
    }

    // actions that don't return will result in a state dump-to-file here
    state.save();
}

fn do_token_show_cmd(state: &State) {
    match &state.session_token {
        Some(token) => println!("{}", token),
        None => eprintln!("Missing token, did you run `arv token set`?"),
    }
}
