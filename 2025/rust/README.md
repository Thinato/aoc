# Advent of Code (Rust)

Rust solutions for Advent of Code puzzles, organized by year. Each year lives in its own Cargo binary crate under `<year>/rust` and exposes the same CLI: pass the day number you want to run.

## Repository layout
- `2024/rust`: crate `aoc2024`; entrypoint `src/main.rs` dispatches to day modules; day 1 implemented.
- `2025/rust`: crate `aoc2025`; entrypoint `src/main.rs` dispatches to day modules; days 1–4 scaffolded (day 4 still in-progress) with placeholders for the rest.
- `inputs/`: per-year folder holding the raw puzzle input files (e.g., `inputs/day1`). Some solutions may read `dayN-test` while being developed.

## Prerequisites
- Rust toolchain with Cargo (recommended: latest stable).
- Run commands from the year folder you want (e.g., `cd 2025/rust`), or use `--manifest-path` from the repo root.

## Running solutions
```bash
# From the repo root
cargo run --manifest-path 2024/rust/Cargo.toml -- 1   # 2024 day 1
cargo run --manifest-path 2025/rust/Cargo.toml -- 2   # 2025 day 2
```
- If you omit the day argument you’ll see a usage hint; invalid day numbers print “not implemented/available.”
- Add your puzzle input to the matching file in `inputs/` before running. Example: place the AoC 2025 day 3 input in `2025/rust/inputs/day3`.

## Implemented status
- 2024: day 1 solved; other days print `not implemented`.
- 2025: days 1–3 solved; day 4 stubbed; days 5–12 currently `not implemented`.

## Adding a new day (Rust template)
1. Create `src/dayN.rs` with a `pub fn execute()` entrypoint.
2. Register the module at the top of `src/main.rs` (`mod dayN;`) and add a `match` arm that calls `dayN::execute()`.
3. Drop your puzzle input in `inputs/dayN` (or `dayN-test` while experimenting).
4. Run with `cargo run -- N` to verify.

## Adding a new year
1. Copy an existing year folder as a starting point (or `cargo new aocYYYY` inside a new `YYYY/rust`).
2. Keep the same CLI shape: parse the day from the first CLI argument and dispatch in `execute`.
3. Repeat the day/module pattern above as you solve puzzles.
