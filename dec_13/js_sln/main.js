const testInput = `
939
7,13,x,x,59,x,31,19
`;

const actualInput = `
1000390
13,x,x,41,x,x,x,x,x,x,x,x,x,997,x,x,x,x,x,x,x,23,x,x,x,x,x,x,x,x,x,x,19,x,x,x,x,x,x,x,x,x,29,x,619,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,17
`;

const input = actualInput;
const [earliestStr, scheduleCsv] = input.split("\n").filter((l) => l.length);
const earliest = parseInt(earliestStr);
const intervals = scheduleCsv
  .split(",")
  .filter((c) => c !== "x")
  .map((c) => parseInt(c));

pt1(earliest, intervals);

function pt1(earliest, intervals) {
  const schedules = new Map();
  for (let line of intervals) {
    const closestTime = Math.ceil(earliest / line) * line;
    const differential = closestTime - earliest;
    schedules.set(line, { closestTime, differential });
  }

  const pt1 = {
    line: Number.MAX_VALUE,
    departure: Number.MAX_VALUE,
    differential: Number.MAX_VALUE,
  };

  for (let line of intervals) {
    const { closestTime, differential } = schedules.get(line);
    if (differential < pt1.differential) {
      pt1.line = line;
      pt1.departure = closestTime;
      pt1.differential = differential;
    }
  }

  console.log(`Pt. 1: ${pt1.line * pt1.differential}`, pt1);
}
