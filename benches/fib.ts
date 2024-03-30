import * as fs from 'fs';

function fibonacci(n: number): number {
  if (n <= 1) {
    return n;
  } else {
    return fibonacci(n - 1) + fibonacci(n - 2);
  }
}

const outputFilePath = 'fib-ts.csv';
let totalTime = 0;

// Open the output file and write the headers
const outputStream = fs.createWriteStream(outputFilePath);
outputStream.write('Run,Time\n');

for (let i = 1; i <= 20; i++) {
  const startTime = Date.now();
  fibonacci(45);
  const endTime = Date.now();
  const executionTime = (endTime - startTime) / 1000; // Convert to seconds
  totalTime += executionTime;


  console.log(`Run ${i} Time taken: ${executionTime} seconds`);
  // Write the current execution time to the output file
  const row = `${i},${executionTime}\n`;
  outputStream.write(row);
}

outputStream.end();

console.log(`Average time taken: ${totalTime/20} s`);
