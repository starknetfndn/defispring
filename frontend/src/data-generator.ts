/// This file can be used to generate test JSON data
/// Run with: `ts-node src/data-generator.ts`

const fs = require("fs");

function zeroPadHex(num: number, width: number): string {
  const hexString = num.toString(16);
  return "0x" + "0".repeat(width - hexString.length) + hexString;
}

function generateJSONFile(numEntries: number): void {
  const data = [];

  for (let i = 1; i <= numEntries; i++) {
    const address = zeroPadHex(i, 64);
    const amount = (1000 + i * 10).toString(); // Auto-incrementing amount

    const entry = {
      address: address,
      amount: amount,
    };

    data.push(entry);
  }

  const jsonString = JSON.stringify(data, null, 2);

  fs.writeFileSync("raw_x.json", jsonString);

  console.log(`Generated ${numEntries} entries. Check 'output.json' file.`);
}

// Specify the number of entries you want to generate
const numEntriesToGenerate: number = 50000;

generateJSONFile(numEntriesToGenerate);
