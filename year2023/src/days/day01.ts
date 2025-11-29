import { readFileSync } from 'node:fs';

export function solve(filePath: string): void {
  const input: string = readFileSync(filePath, 'utf-8');
  const lines: string[] = input.trim().split('\n');

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
      console.log(regexMatches)
      calibrationValueSum += +regexMatches[0].concat(regexMatches[regexMatches.length - 1])
    } else {
      console.log(`No match for line ${i}`)
    }
  }

  return calibrationValueSum;
}

function solvePart2(lines: string[]): number {
/*  const partTwoInput = input.replaceAll('one', '1')
                            .replaceAll('two', '2')
                            .replaceAll('three', '3')
                            .replaceAll('four', '4')
                            .replaceAll('five', '5')
                            .replaceAll('six', '6')
                            .replaceAll('seven', '7')
                            .replaceAll('eight', '8')
                            .replaceAll('nine', '9'); 
  const partTwoLines: string[] = partTwoInput.trim().split('\n'); */
  const numberPattern: RegExp = new RegExp(/\d|one|two|three|four|five|six|seven|eight|nine/g);
  let calibrationValueSum: number = 0; 

  for (let i = 0; i < lines.length; i++) {
    let first: number;
    let last: number;
    const regexMatches: RegExpMatchArray | null = lines[i].match(numberPattern);
    if (regexMatches) {
      console.log(regexMatches)
      const first = replaceIfTextNumber(regexMatches[0])
      console.log(first)
      const last = replaceIfTextNumber(regexMatches[regexMatches.length - 1])
      console.log(last)
      calibrationValueSum += +first.concat(last)
    } else {
      console.log(`No match for line ${i}`)
    }
    // TODO: test reverse solution. Need to reverse numberPattern literal numbers as well.
    // Rationale: solution too low due to overlapping text numbers at end of string / last
    // literal number not being caught correctly
    // const reverseRegexMatches: RegExpMatchArray | null = lines[i]
  }

  return calibrationValueSum;
}

function replaceIfTextNumber(maybeNumber: string): string {
  switch (maybeNumber) {
    case 'one': return '1';
    case 'two': return '2';
    case 'three': return '3';
    case 'four': return '4';
    case 'five': return '5';
    case 'six': return '6';
    case 'seven': return '7';
    case 'eight': return '8';
    case 'nine': return '9';
    default: return maybeNumber
  }
}
