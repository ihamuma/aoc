import { readFileSync } from 'node:fs';

export function solve(filePath: string): void {
  const input = readFileSync(filePath, 'utf-8');
  const lines = input.trim().split('\n');

  // Part 1
  const part1 = solvePart1(lines);
  console.log('Part 1 calibration values sum:', part1);

  // Part 2
  const part2 = solvePart2(lines);
  console.log('Part 2:', part2);
}

function solvePart1(lines: string[]): number {

  const numberPattern: RegExp = new RegExp(/\d/g);

  let calibrationValueSum: number = 0; 

  for (let i = 0; i < lines.length; i++) {
    const regexMatches: RegExpMatchArray | null = lines[i].match(numberPattern);
    if (regexMatches) {
      calibrationValueSum += +regexMatches[0].concat(regexMatches[regexMatches.length - 1])
    } else {
      console.log(`No match for line ${i}`)
    }
  }

  return calibrationValueSum;
}

function solvePart2(lines: string[]): string {
  // TODO: Implement part 2
  return 'Not implemented';
}
