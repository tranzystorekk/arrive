set shell := ["bash", "-c"]

[arg("version", pattern="^[0-9]+\\.[0-9]+\\.[0-9]+")]
prepare_release version:
    @[[ -z "$(git status --porcelain)" ]] || { echo "Error: git worktree is dirty" >&2; exit 1; }

    git cliff -t "{{ version }}" > CHANGELOG.md
    cargo set-version {{ version }}
    cargo check

    git add :/
    git commit -m "chore(version): release {{ version }}"
    git tag -m "version {{ version }}" "v{{ version }}"
