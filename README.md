# Advent of Code

This repository contains solutions for [Advent of Code](https://adventofcode.com/). Solutions are implemented in different languages across years. All should be correct, none are likely to be particularly fast. Enjoy?

## Current Status

- **2024**: 10 days completed (Rust)
- **2022**: 6 days completed (Python)

## Structure

```
aoc/
├── year2024/       # 2024 solutions in Rust
│   ├── src/
│   │   ├── days/       # Solutions for each day
│   ├── input/          # Puzzle inputs (01.txt, 02.txt, etc.)
│   └── input_test/     # Test inputs
├── year2022/       # 2022 solutions in Python
│   ├── src/            # Python solution files (day01.py, day02.py, etc.)
│   ├── input/          # Puzzle inputs (01.txt, 02.txt, etc.)
│   └── input_test/     # Test inputs
└── ...
```

## Running Solutions

### Rust Solutions (2024)

To run any Rust solution, use the following command from the repository root:

```bash
cargo run -p yearYYYY <day> [test]
```

#### Examples

Run 2024 day 1 with real input:
```bash
cargo run -p year2024 1
```

Run 2024 day 1 with test input:
```bash
cargo run -p year2024 1 test
```

### Python Solutions (2022)

To run Python solutions, execute the day file directly:

```bash
python year2022/src/dayXX.py
```

#### Examples

Run 2022 day 1:
```bash
python year2022/src/day01.py
```

## Adding a New Year

To add solutions for a new year:

1. Create a new directory: `yearYYYY/`
2. Copy the structure from an existing year
3. Add the new year to the workspace members in the root `Cargo.toml`
4. Place input files in `yearYYYY/input/` and `yearYYYY/input_test/`
