#!/usr/bin/env node

import { execSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rootDir = join(__dirname, '..');

const args = process.argv.slice(2);

// Help message
function showHelp() {
  console.log(`
Advent of Code CLI

Usage:
  aoc <year> <day> [test]

Arguments:
  year    Year of the puzzle (2022-2024)
  day     Day number (1-25)
  test    Optional flag to use test input instead of real input

Examples:
  aoc 2024 1        Run 2024 day 1 with real input
  aoc 2024 1 test   Run 2024 day 1 with test input
  aoc 2023 5        Run 2023 day 5 with real input
  aoc 2022 3        Run 2022 day 3 with real input

Available years:
  2024 - Rust
  2023 - TypeScript
  2022 - Python
`);
}

// Parse arguments
if (args.length === 0 || args[0] === '--help' || args[0] === '-h') {
  showHelp();
  process.exit(0);
}

if (args.length < 2) {
  console.error('Error: Missing required arguments\n');
  showHelp();
  process.exit(1);
}

const year = args[0];
const day = args[1];
const isTest = args[2] === 'test';

// Validate year
const validYears = ['2022', '2023', '2024'];
if (!validYears.includes(year)) {
  console.error(`Error: Invalid year '${year}'. Valid years: ${validYears.join(', ')}`);
  process.exit(1);
}

// Validate day
const dayNum = parseInt(day, 10);
if (isNaN(dayNum) || dayNum < 1 || dayNum > 25) {
  console.error(`Error: Invalid day '${day}'. Day must be between 1 and 25`);
  process.exit(1);
}

// Build and execute command based on year
let command;

try {
  switch (year) {
    case '2024':
      command = `cargo run -p year2024 ${day}${isTest ? ' test' : ''}`;
      break;

    case '2023':
      command = `npm run solve --prefix year2023 -- ${day}${isTest ? ' test' : ''}`;
      break;

    case '2022':
      const paddedDay = day.padStart(2, '0');
      command = `python3 year2022/src/day${paddedDay}.py`;
      if (isTest) {
        console.warn('Warning: Test mode is not currently supported for 2022 solutions\n');
      }
      break;

    default:
      console.error(`Error: Year ${year} not implemented yet`);
      process.exit(1);
  }

  console.log(`Running: ${command}\n`);

  // Execute the command
  execSync(command, {
    cwd: rootDir,
    stdio: 'inherit',
    env: process.env
  });

} catch (error) {
  // execSync will already output the error, just exit with error code
  process.exit(error.status || 1);
}
