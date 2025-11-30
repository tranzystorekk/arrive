use std::fs::File;
use std::io::{Read, Write};

use eyre::{ensure, OptionExt, Result, WrapErr};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use yansi::Paint;

use crate::paths;

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Stage {
    First,
    Second,
    Complete,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Day {
    pub year: u32,
    pub day: u32,
    pub stage: Stage,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub year: u32,
    pub day: u32,
    pub session_token: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub days: Vec<Day>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            day: 1,
            year: 2015,
            session_token: Default::default(),
            days: Default::default(),
        }
    }
}

impl Stage {
    pub fn advance(&mut self) {
        *self = match self {
            Stage::First => Stage::Second,
            Stage::Second => Stage::Complete,
            Stage::Complete => return,
        }
    }
}

impl State {
    pub fn load() -> Result<Self> {
        let state_dir = paths::state_directory()?;
        std::fs::create_dir_all(&state_dir).wrap_err_with(|| {
            format!(
                "Failed to create state direcotry structure: {}",
                state_dir.display()
            )
        })?;

        let state_file = state_dir.join(paths::STATE_FILE);

        let mut f = File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&state_file)
            .wrap_err_with(|| format!("Failed to open state file: {}", state_file.display()))?;

        let mut s = String::new();
        f.read_to_string(&mut s)
            .wrap_err("Failed to read state file")?;

        let result = if s.is_empty() {
            let v = Default::default();
            let serialized =
                toml::to_string(&v).wrap_err("Failed to serialize new default state")?;
            f.write_all(serialized.as_bytes())
                .wrap_err("Failed to write new default state to file")?;
            v
        } else {
            let v: Self = toml::from_str(&s).wrap_err("Failed to deserialize state")?;
            v.validate()?;
            v
        };

        Ok(result)
    }

    pub fn save(&self) -> Result<()> {
        let state_dir = paths::state_directory()?;
        std::fs::create_dir_all(&state_dir).wrap_err_with(|| {
            format!(
                "Failed to create state directory structure: {}",
                state_dir.display()
            )
        })?;

        let state_file = state_dir.join(paths::STATE_FILE);

        let serialized = toml::to_string(self).wrap_err("Failed to serialize state")?;
        std::fs::write(&state_file, serialized)
            .wrap_err_with(|| format!("Failed to write state to file: {}", state_file.display()))?;

        Ok(())
    }

    pub fn session_token(&self) -> Result<&str> {
        self.session_token
            .as_deref()
            .ok_or_eyre("Missing session token, have you run `arv token set`?")
    }

    pub fn print_status(&self) {
        println!("selected: {}/{:02}", self.year, self.day);

        if self.days.is_empty() {
            return;
        }

        println!();

        let years = self
            .days
            .iter()
            .cloned()
            .sorted_by_key(|d| (d.year, d.day))
            .chunk_by(|d| d.year);

        for (year, chunk) in &years {
            println!("{}", year);

            for d in chunk {
                let symbol = match d.stage {
                    Stage::First => "__".to_string(),
                    Stage::Second => format!("{}_", "*".bold().bright_white()),
                    Stage::Complete => format!(
                        "{}{}",
                        "*".bold().bright_white(),
                        "*".bold().bright_yellow()
                    ),
                };
                println!("     {:02} {}", d.day, symbol);
            }
        }
    }

    fn validate(&self) -> Result<()> {
        ensure!(
            (2015..).contains(&self.year),
            "Invalid year selected: {}",
            self.year
        );
        ensure!(
            (1..=25).contains(&self.day),
            "Invalid day selected: {}",
            self.day
        );

        for d in &self.days {
            ensure!(
                (2015..).contains(&d.year),
                "Invalid solution state year: {}",
                d.year
            );
            ensure!(
                (1..=25).contains(&d.day),
                "Invalid solution state day: {}",
                d.day
            );
        }

        Ok(())
    }
}
