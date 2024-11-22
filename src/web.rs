use eyre::{bail, Result, WrapErr};

use crate::state::{Day, Stage, State};

const AOC_BASE_URL: &str = "https://adventofcode.com";
const HTTP_OK: i32 = 200;

fn user_agent() -> String {
    let repo = env!("CARGO_PKG_REPOSITORY");
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");

    format!("{}@{} by {}", repo, version, authors)
}

pub fn fetch_input(state: &State, year: u32, day: u32) -> Result<String> {
    let url = format!("{}/{}/day/{}/input", AOC_BASE_URL, year, day);
    let user_agent = user_agent();
    let session_token = state.session_token()?;

    let session_cookie = format!("session={}", session_token);

    let req = minreq::get(url)
        .with_header("Cookie", session_cookie)
        .with_header("User-Agent", user_agent);

    let response = req.send().wrap_err("Request for input file failed")?;

    if response.status_code != HTTP_OK {
        eprint!("{}", response.as_str()?);
        bail!("HTTP {} {}", response.status_code, response.reason_phrase);
    }

    let input = response.as_str()?;

    Ok(input.to_string())
}

pub fn submit(state: &mut State, solution: &str) -> Result<()> {
    let url = format!("{}/{}/day/{}/answer", AOC_BASE_URL, state.year, state.day);
    let user_agent = user_agent();
    let session_token = state.session_token()?;

    let session_cookie = format!("session={}", session_token);

    let maybe_d = state
        .days
        .iter_mut()
        .find(|d| d.year == state.year && d.day == state.day);
    let target_day = match maybe_d {
        Some(day) => day,
        None => {
            let new_day = Day {
                year: state.year,
                day: state.day,
                stage: Stage::First,
            };
            state.days.push(new_day);
            state.days.last_mut().unwrap()
        }
    };

    let level = match target_day.stage {
        Stage::First => 1,
        Stage::Second => 2,
        Stage::Complete => bail!(
            "You have already completed selected day: {}/{:02}",
            state.year,
            state.day
        ),
    };
    let form_body = format!("level={}&answer={}", level, solution).into_bytes();

    let req = minreq::post(url)
        .with_header("Cookie", session_cookie)
        .with_header("User-Agent", user_agent)
        .with_header("Content-Type", "application/x-www-form-urlencoded")
        .with_body(form_body);

    let response = req
        .send()
        .wrap_err("Request with solution submission failed")?;

    if response.status_code != HTTP_OK {
        eprint!("{}", response.as_str()?);
        bail!("HTTP {} {}", response.status_code, response.reason_phrase);
    }

    let response_body = response.as_str()?;

    if response_body.contains("That's the right answer!") {
        target_day.stage.advance();
    }

    eprintln!("{}", response_body);

    Ok(())
}
