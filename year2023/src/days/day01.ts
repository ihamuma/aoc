import { readFileSync } from 'node:fs';

export function solve(filePath: string): void {
  const input = readFileSync(filePath, 'utf-8');
  const lines = input.trim().split('\n');

  // Part 1
  const part1 = solvePart1(lines);
  console.log('Part 1:', part1);

  // Part 2
  const part2 = solvePart2(lines);
  console.log('Part 2:', part2);
}

function solvePart1(lines: string[]): number {
  const results: number[] = [];
  lines.forEach((line) => {
    console.log(line.split(''))
  })
  return 0;
}

function solvePart2(lines: string[]): string {
  // TODO: Implement part 2
  return 'Not implemented';
}
