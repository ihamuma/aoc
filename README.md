# Advent of Code

This repository contains solutions for [Advent of Code](https://adventofcode.com/).
Solutions are implemented in different languages across years.
All should be correct, none are likely to be particularly fast. Enjoy?

## Current Status

- **2025**: 2 days completed (Rust)
- **2024**: 9 days completed (Rust)
- **2023**: 1 day completed (TypeScript)
- **2022**: 6 days completed (Python)

## Structure

```
aoc/
├── bin/
│   └── aoc.js          # Unified CLI dispatcher
├── year2024/           # 2024 solutions in Rust
│   ├── src/
│   │   ├── days/       # Solutions for each day
│   ├── input/          # Puzzle inputs (01.txt, 02.txt, etc.)
│   └── input_test/     # Test inputs
├── year2023/           # 2023 solutions in TypeScript
│   ├── src/
│   │   ├── days/       # Solutions for each day
│   ├── input/          # Puzzle inputs (01.txt, 02.txt, etc.)
│   └── input_test/     # Test inputs
├── year2022/           # 2022 solutions in Python
│   ├── src/            # Python solution files (day01.py, day02.py, etc.)
│   ├── input/          # Puzzle inputs (01.txt, 02.txt, etc.)
│   └── input_test/     # Test inputs
└── ...
```

## Running Solutions

### Unified CLI (Recommended)

The repository includes a cross-platform CLI tool that works with all years. This is the recommended way to run solutions.

#### Installation

Install the CLI globally (one-time setup):

```bash
npm link
```

This makes the `aoc` command available globally from anywhere on your system.

#### Usage

```bash
aoc <year> <day> [test]
```

**Examples:**

```bash
aoc 2024 1        # Run 2024 day 1 with real input
aoc 2024 1 test   # Run 2024 day 1 with test input
aoc 2023 5        # Run 2023 day 5 with real input
aoc 2022 3 test   # Run 2022 day 3 with test input (warning: not supported)
```

**Without global installation:**

If you prefer not to install globally, you can run the CLI directly:

```bash
./bin/aoc.js 2024 1 test
```

#### Help

```bash
aoc --help
```

### Language-Specific Instructions

If you prefer to run solutions directly without the CLI:

#### Rust Solutions (2024)

To run any Rust solution, use the following command from the repository root:

```bash
cargo run -p year2024 <day> [test]
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

#### TypeScript Solutions (2023)

To run TypeScript solutions, use npm from the year2023 directory or from the repository root:

```bash
npm run solve --prefix year2023 -- <day> [test]
```

**Examples:**

Run 2023 day 1 with real input:
```bash
npm run solve --prefix year2023 -- 1
```

Run 2023 day 1 with test input:
```bash
npm run solve --prefix year2023 -- 1 test
```

Or from within the year2023 directory:
```bash
cd year2023
npm run solve -- 1
npm run solve -- 1 test
```

#### Python Solutions (2022)

To run Python solutions, execute the day file directly:

```bash
python3 year2022/src/dayXX.py
```

**Note:** Test mode is not currently supported for 2022 solutions.

**Examples:**

Run 2022 day 1:
```bash
python3 year2022/src/day01.py
```

## Adding a New Year

To add solutions for a new year:

1. Create a new directory: `yearYYYY/`
2. Copy the structure from an existing year (choose based on your preferred language)
3. For Rust years only: Add the new year to the workspace members in the root `Cargo.toml`
4. For TypeScript years: Run `npm install` in the year directory after setup
5. Place input files in `yearYYYY/input/` and `yearYYYY/input_test/`
6. Update the CLI dispatcher in `bin/aoc.js` to add a case for the new year
7. Update this README to reflect the new year in the Current Status section
