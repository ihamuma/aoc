import * as day01 from './days/day01.ts';

const args = process.argv.slice(2);

if (args.length < 1) {
  console.error('Usage: npm run solve -- <day> [test]');
  process.exit(1);
}

const day = parseInt(args[0], 10);
const isTest = args[1] === 'test';

if (isNaN(day) || day < 1 || day > 25) {
  console.error(`Invalid day: ${args[0]}`);
  process.exit(1);
}

const filePath = isTest
  ? `input_test/${day.toString().padStart(2, '0')}.txt`
  : `input/${day.toString().padStart(2, '0')}.txt`;

const startTime = performance.now();

switch (day) {
  case 1:
    day01.solve(filePath);
    break;
  // Add more cases as you implement more days
  default:
    console.log(`Day ${day} not implemented yet.`);
    process.exit(0);
}

const elapsed = performance.now() - startTime;
console.log(`\nDay ${day}${isTest ? ' test' : ''} ran in ${elapsed.toFixed(2)}ms`);
