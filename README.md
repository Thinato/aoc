# Advent of Code

Collection of Advent of Code solutions organized by year and language. Each year lives in its own folder and can host multiple implementations; inputs for each solver live alongside the code under an `inputs/` directory.

## Repository layout
- `2024/rust/` — Rust crate `aoc2024`; CLI expects the day number as an argument.
- `2025/rust/` — Rust crate `aoc2025`; similar CLI interface for day selection.
- `2025/readme.md` — short year-specific note about the 2025 event.

## Running solutions
From the year/language folder:

```bash
# Examples
cd 2024/rust
cargo run --release -- 1    # run day 1 for 2024

cd ../../2025/rust
cargo run --release -- 4    # run day 4 for 2025
```

Inputs are read from `inputs/dayN` (with optional `dayN-test` files for samples). Drop your puzzle input into the matching file before running a day.

## Adding more years or languages
- Create a new year folder (e.g., `2026/`) and add subfolders per language.
- Keep inputs under `year/lang/inputs/` and follow the existing CLI pattern of accepting the day number as the first argument.
- Add a small year-level README if there are quirks for that edition.
