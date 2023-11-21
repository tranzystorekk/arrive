# Arrive

[![GitHub release (latest SemVer)](https://badgers.space/github/release/tranzystorekk/arrive)](https://github.com/tranzystorekk/arrive/releases/latest)
[![Crates.io](https://badgers.space/crates/version/arrive?icon=feather-package)](https://crates.io/crates/arrive)

## About

A simple tool to fetch your Advent of Code input and submit your solutions.

Key features and goals:

- keeps your current AOC day selection and solution status in a TOML state file (for human readability)
- caches input files to ease load on AOC servers
- reasonably minimal dependencies

Directories used (see [dirs](https://docs.rs/dirs/latest/dirs/index.html) for more info):

|Description|Path                                                       |
|-----------|-----------------------------------------------------------|
|Cache      |`${XDG_CACHE_HOME}/arrive` or `${HOME}/.cache/arrive`      |
|State      |`${XDG_STATE_HOME}/arrive` or `${HOME}/.local/state/arrive`|

## Install

Via Cargo:

```console
cargo install --locked arrive
```

From source:

```console
git clone https://github.com/tranzystorekk/arrive.git
cargo install --path arrive
```

## Usage

Set a new session token (see [Token](#token) section):

```console
arv token set <token>
```

Print currently stored session token:

```console
arv token show
```

Select currently processed advent day:

```console
arv select -y <year> -d <day>
```

Show current selection and day completion status:

```console
arv status
```

Print input for current selection:

```console
arv input
```

Save input to file:

```console
arv input > path/to/file.txt
```

Submit advent solution for current selection.
If `<solution>` is omitted, STDIN is read instead.

```console
arv submit [<solution>]
```

## Token

For the web commands (`arv input`, `arv submit`) to work,
a valid AOC session token needs to be supplied to `arv token set`.

To obtain such a token, do the following:

1. Login to AOC: <https://adventofcode.com/auth/login>
2. In your browser, open the developer tools
3. Navigate to Storage -> Cookies
4. Find and copy the entry named `session`
5. Run `arv token set <token>`

These tokens have long expiration dates,
so you can typically set-and-forget once.

## Acknowledgements

Deeply inspired by [aocf](https://github.com/nuxeh/aocf)!
