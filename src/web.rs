use std::fmt::Write;

use eyre::{bail, OptionExt, Result, WrapErr};
use tl::{parse, ParserOptions};

use crate::state::{Day, Stage, State};

const AOC_BASE_URL: &str = "https://adventofcode.com";
const HTTP_OK: i32 = 200;

fn user_agent() -> String {
    let repo = env!("CARGO_PKG_REPOSITORY");
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");

    format!("{}@{} by {}", repo, version, authors)
}

fn parse_response(response_body: &str) -> Result<String> {
    let dom = parse(response_body, ParserOptions::default())?;
    let parser = dom.parser();

    let main_tag = dom
        .nodes()
        .iter()
        .filter_map(|node| node.as_tag())
        .find(|tag| tag.name() == "main")
        .ok_or_eyre("No <main> tag found")?;

    let mut text = main_tag.inner_text(parser).to_string();
    let links: Vec<_> = main_tag
        .query_selector(parser, "a[href]")
        .map(|anchor_nodes| {
            anchor_nodes
                .filter_map(|anchor| anchor.get(parser).and_then(|node| node.as_tag()))
                .filter_map(|element| {
                    element
                        .attributes()
                        .get("href")
                        .flatten()
                        .map(|href| (href.as_utf8_str(), element.inner_text(parser).to_string()))
                        .filter(|(_, text)| !text.is_empty())
                })
        })
        .into_iter()
        .flatten()
        .collect();

    for (i, (_, link_text)) in links.iter().enumerate() {
        let replaced = format!("({})[{}]", link_text, i + 1);
        text = text.replace(link_text, &replaced);
    }

    let mut output = String::new();

    writeln!(&mut output, "{}", text.trim())?;
    writeln!(&mut output)?;

    for (i, (href, _)) in links.iter().enumerate() {
        let link = if href.starts_with("http") {
            href.to_string()
        } else {
            format!("{}{}", AOC_BASE_URL, href)
        };

        writeln!(&mut output, "[{}]: {}", i + 1, link)?;
    }

    Ok(output)
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

    match parse_response(response_body) {
        Ok(message) => eprint!("{}", message),
        Err(report) => {
            eprintln!("Failed to parse response with error:");
            eprintln!("------------------------------------");
            eprintln!("{:?}", report);
            eprintln!("------------------------------------");
            eprintln!();
            eprintln!("Falling back to raw response below");
            eprintln!();
            eprintln!("{}", response_body.trim());
        }
    }

    Ok(())
}
