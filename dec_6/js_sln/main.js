const fs = require("fs");

const data = fs.readFileSync("/Users/ianjones/Desktop/day6").toString();

const grouped = data.split("\n\n");

const pt1 = grouped
  .map((g) => new Set(g.split("\n").join("").split("")))
  .map((s) => Array.from(s).length)
  .reduce((acc, next) => (acc = acc + next));

const pt2 = grouped
  .map((g) => g.split("\n").filter((s) => Boolean(s)))
  .map((arr) => arr.map((s) => s.split("")))
  .map(toIntersections)
  .map((inter) => inter.length)
  .reduce((acc, next) => (acc = acc + next));

function toIntersections(arrArr) {
  const sets = [];
  for (let arr of arrArr) {
    sets.push(new Set(arr));
  }
  return Array.from(
    sets.reduce((acc, next) => acc.intersection(next)).values()
  );
}

console.log(`Pt 1: ${pt1}\n Pt 2: ${pt2}`);
