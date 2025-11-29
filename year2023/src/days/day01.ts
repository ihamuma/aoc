import { lstat, readFileSync } from 'node:fs';

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
  const numberPattern: RegExp =        new RegExp(/\d|one|two|three|four|five|six|seven|eight|nine/);
  const reverseNumberPattern: RegExp = new RegExp(/\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin/);
  let calibrationValueSum: number = 0; 

  for (let i = 0; i < lines.length; i++) {
    let first: string = '0';
    let last: string = '0';

    const regexMatches: RegExpMatchArray | null = lines[i].match(numberPattern);
    const reverseRegexMatches: RegExpMatchArray | null = reverseString(lines[i]).match(reverseNumberPattern);

    if (regexMatches) {
      first = replaceIfTextNumber(regexMatches[0])
    }
    if (reverseRegexMatches) {
      last = replaceIfTextNumber(reverseString(reverseRegexMatches[0]))
    }

    calibrationValueSum += +first.concat(last)
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

function reverseString(str: string): string {
  return str.split('').reverse().join('')
}
