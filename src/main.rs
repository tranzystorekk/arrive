mod cache;
mod cli;
mod paths;
mod state;
mod web;

use std::io::{Read, Write};

use cache::Entry;
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
        Cli::Input(args) => {
            do_input_cmd(&state, args);
            return;
        }
        Cli::Submit { solution } => {
            do_submit(&mut state, solution);
        }
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

fn do_input_cmd(state: &State, args: InputArgs) {
    let (year, day) = match (args.year, args.day) {
        (Some(y), Some(d)) => (y, d),
        _ => (state.year, state.day),
    };

    if args.force {
        let contents = web::fetch_input(state, year, day);
        cache::force_write(year, day, contents.as_bytes());
        println!("{}", contents);
        return;
    }

    match cache::fetch(year, day) {
        Entry::Cached(mut file) => {
            let mut buf = String::new();
            file.read_to_string(&mut buf).unwrap();
            println!("{}", buf);
        }
        Entry::Missing(mut file) => {
            let contents = web::fetch_input(state, year, day);
            file.write_all(contents.as_bytes()).unwrap();
            println!("{}", contents);
        }
    }
}

fn do_submit(state: &mut State, solution: Option<String>) {
    let solution = match solution {
        Some(solution) => solution,
        None => {
            let mut result = String::new();
            std::io::stdin().read_to_string(&mut result).unwrap();

            result.trim().to_string()
        }
    };

    web::submit(state, &solution);
}

fn do_token_show_cmd(state: &State) {
    match &state.session_token {
        Some(token) => println!("{}", token),
        None => eprintln!("Missing token, did you run `arv token set`?"),
    }
}
