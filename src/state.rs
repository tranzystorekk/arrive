use std::fs::File;
use std::io::{Read, Write};

use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};

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
        std::fs::create_dir_all(&state_dir).with_context(|| {
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
            .open(&state_file)
            .with_context(|| format!("Failed to open state file: {}", state_file.display()))?;

        let mut s = String::new();
        f.read_to_string(&mut s)
            .context("Failed to read state file")?;

        let result = if s.is_empty() {
            let v = Default::default();
            let serialized =
                toml::to_string(&v).context("Failed to serialize new default state")?;
            f.write_all(serialized.as_bytes())
                .context("Failed to write new default state to file")?;
            v
        } else {
            let v: Self = toml::from_str(&s).context("Failed to deserialize state")?;
            v.validate()?;
            v
        };

        Ok(result)
    }

    pub fn save(&self) -> Result<()> {
        let state_dir = paths::state_directory()?;
        std::fs::create_dir_all(&state_dir).with_context(|| {
            format!(
                "Failed to create state directory structure: {}",
                state_dir.display()
            )
        })?;

        let state_file = state_dir.join(paths::STATE_FILE);

        let serialized = toml::to_string(self).context("Failed to serialize state")?;
        std::fs::write(&state_file, serialized)
            .with_context(|| format!("Failed to write state to file: {}", state_file.display()))?;

        Ok(())
    }

    pub fn session_token(&self) -> Result<&str> {
        self.session_token
            .as_deref()
            .ok_or_else(|| anyhow!("Missing session token, have you run `arv token set`?"))
    }

    pub fn print_status(&self) {
        println!("selected: {}/{:02}", self.year, self.day);

        if self.days.is_empty() {
            return;
        }

        println!();

        let mut sorted = self.days.clone();
        sorted.sort_by_key(|d| (d.year, d.day));

        for d in &sorted {
            let symbol = match d.stage {
                Stage::First => "☐☐",
                Stage::Second => "🗹☐",
                Stage::Complete => "🗹🗹",
            };
            println!("{}/{:02} {}", d.year, d.day, symbol);
        }
    }

    fn validate(&self) -> Result<()> {
        if !(2015..).contains(&self.year) {
            bail!("Invalid year selected: {}", self.year);
        }

        if !(1..=25).contains(&self.day) {
            bail!("Invalid day selected: {}", self.day);
        }

        for d in &self.days {
            if !(2015..).contains(&d.year) {
                bail!("Invalid solution state year: {}", d.year);
            }

            if !(1..=25).contains(&d.day) {
                bail!("Invalid solution state day: {}", d.day);
            }
        }

        Ok(())
    }
}
