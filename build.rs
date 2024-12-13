use std::ffi::OsString;
use std::io::Error;
use std::{env, path::PathBuf};

use clap_complete::aot;
use clap_complete::generate_to;
use clap_complete::Generator;

include!("src/cli.rs");

const BIN_NAME: &str = "arv";

fn generate_completion(
    gen: impl Generator,
    out_dir: impl Into<OsString>,
) -> Result<PathBuf, Error> {
    use clap::CommandFactory;
    generate_to(gen, &mut Cli::command(), BIN_NAME, out_dir)
}

fn main() {
    let out_dir = env::var_os("ARRIVE_ASSETS_GEN_DIR")
        .or_else(|| env::var_os("OUT_DIR"))
        .map(PathBuf::from)
        .expect("neither ARRIVE_ASSETS_GEN_DIR nor OUT_DIR was set, cannot generate completions");

    let completion_assets_dir = out_dir.join("assets/completions");
    std::fs::create_dir_all(&completion_assets_dir)
        .expect("failed to create completions asset dir");

    generate_completion(aot::Bash, &completion_assets_dir)
        .expect("failed to generate bash completion");
    generate_completion(aot::Fish, &completion_assets_dir)
        .expect("failed to generate fish completion");
    generate_completion(aot::Zsh, &completion_assets_dir)
        .expect("failed to generate zsh completion");
    generate_completion(aot::Elvish, &completion_assets_dir)
        .expect("failed to generate elvish completion");
    generate_completion(aot::PowerShell, &completion_assets_dir)
        .expect("failed to generate powershell completion");
}
