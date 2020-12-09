const fs = require("fs");
const data = fs.readFileSync("/Users/ianjones/aoc2020/dec_9/input").toString();

const preambleLen = 25;
const input = data
  .split("\n")
  .filter((l) => l.length > 0)
  .map((l) => parseInt(l));

const invalid = getFirstInvalid(input, preambleLen);
console.log("Pt 1: " + invalid);
const minMax = findContiguousSumMinMax(input, invalid);
console.log("Pt 2", { answer: minMax.lower + minMax.upper });

function getFirstInvalid(input, preambleLen) {
  for (let i = preambleLen; i < input.length; i++) {
    const targetSet = new Set(input.slice(i - preambleLen, i));
    const current = input[i];
    const matches = [];
    for (let j of targetSet) {
      const x = current - j;
      if (targetSet.has(x)) {
        matches.push(x);
      }
    }
    if (matches.length < 2) {
      return current;
    }
  }
}

function findContiguousSumMinMax(input, targetNum) {
  const upperIndex = input.indexOf(targetNum);
  const range = input.slice(0, upperIndex);
  for (let i = 0; i < range.length; i++) {
    const currentI = range[i];
    const resultArr = [currentI];
    let sum = currentI;
    for (let j = i + 1; j < range.length; j++) {
      if (sum === targetNum) {
        return {
          resultArr,
          lower: Math.min(...resultArr),
          upper: Math.max(...resultArr),
        };
      }
      if (sum < targetNum) {
        const currentJ = range[j];
        sum += currentJ;
        resultArr.push(currentJ);
      } else {
        break;
      }
    }
  }
}
