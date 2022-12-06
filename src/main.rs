mod cache;
mod cli;
mod paths;
mod state;
mod web;

use std::io::{Read, Write};

use anyhow::{Context, Result};
use clap::Parser;

use cache::Entry;
use cli::{Cli, InputArgs, TokenCmd};
use state::State;

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Cli::Completion { shell } = cli {
        cli::generate_completion(shell);
        return Ok(());
    }

    let mut state = State::load()?;

    match cli {
        Cli::Token(TokenCmd::Set { token }) => state.session_token = Some(token),
        Cli::Token(TokenCmd::Show) => {
            do_token_show_cmd(&state)?;
            return Ok(());
        }
        Cli::Input(args) => {
            do_input_cmd(&state, args)?;
            return Ok(());
        }
        Cli::Submit { solution } => {
            do_submit(&mut state, solution)?;
        }
        Cli::Select { year, day } => {
            state.year = year;
            state.day = day;
        }
        Cli::Status => {
            state.print_status();
            return Ok(());
        }
        Cli::Completion { .. } => unreachable!(),
    }

    // actions that don't return will result in a state dump-to-file here
    state.save()?;

    Ok(())
}

fn print_content(content: &str) -> Result<()> {
    print!("{}", content);
    std::io::stdout().flush()?;

    Ok(())
}

fn do_input_cmd(state: &State, args: InputArgs) -> Result<()> {
    let (year, day) = match (args.year, args.day) {
        (Some(y), Some(d)) => (y, d),
        _ => (state.year, state.day),
    };

    if args.force {
        let contents = web::fetch_input(state, year, day)?;
        print_content(&contents)?;
        cache::force_write(year, day, contents.as_bytes())?;
        return Ok(());
    }

    match cache::fetch(year, day)? {
        Entry::Cached(mut file) => {
            let mut buf = String::new();
            file.read_to_string(&mut buf)
                .context("Failed to read cached input file")?;
            print_content(&buf)?;
        }
        Entry::Missing(mut file) => {
            let contents = web::fetch_input(state, year, day)?;
            print_content(&contents)?;
            file.write_all(contents.as_bytes())
                .context("Failed to write input file to cache")?;
        }
    }

    Ok(())
}

fn do_submit(state: &mut State, solution: Option<String>) -> Result<()> {
    let solution = match solution {
        Some(solution) => solution,
        None => {
            let mut result = String::new();
            std::io::stdin()
                .read_to_string(&mut result)
                .context("Failed to read solution from STDIN")?;

            result.trim().to_string()
        }
    };

    web::submit(state, &solution)?;

    Ok(())
}

fn do_token_show_cmd(state: &State) -> Result<()> {
    let token = state.session_token()?;

    println!("{}", token);

    Ok(())
}
