use std::fs::File;
use std::io::{Read, Write};

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
    pub fn load() -> Self {
        let state_dir = paths::state_directory().unwrap();
        std::fs::create_dir_all(&state_dir).unwrap();

        let state_file = state_dir.join(paths::STATE_FILE);

        let mut f = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(&state_file)
            .unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();

        if s.is_empty() {
            let v = Default::default();
            f.write_all(&toml::to_vec(&v).unwrap()).unwrap();
            v
        } else {
            toml::from_str(&s).unwrap()
        }
    }

    pub fn save(&self) {
        let state_dir = paths::state_directory().unwrap();
        std::fs::create_dir_all(&state_dir).unwrap();

        let state_file = state_dir.join(paths::STATE_FILE);

        std::fs::write(state_file, toml::to_vec(self).unwrap()).unwrap();
    }

    pub fn print_status(&self) {
        println!("year = {}", self.year);
        println!("day = {}", self.day);

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
}
