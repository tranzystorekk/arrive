use crate::state::State;

const AOC_BASE_URL: &str = "https://adventofcode.com";

fn user_agent() -> String {
    let repo = env!("CARGO_PKG_REPOSITORY");
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");

    format!("{}@{} by {}", repo, version, authors)
}

pub fn fetch_input(state: &State, year: u32, day: u32) -> String {
    let url = format!("{}/{}/day/{}/input", AOC_BASE_URL, year, day);
    let user_agent = user_agent();

    let session_cookie = format!("session={}", state.session_token.as_deref().unwrap());

    let req = minreq::get(url)
        .with_header("Cookie", session_cookie)
        .with_header("User-Agent", user_agent);

    let response = req.send().unwrap();

    eprintln!("{}", response.as_str().unwrap());

    todo!()
}

pub fn submit(state: &State, solution: &str) {
    let url = format!("{}/{}/day/{}", AOC_BASE_URL, state.year, state.day);

    todo!()
}
