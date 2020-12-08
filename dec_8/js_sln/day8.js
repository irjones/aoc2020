const fs = require("fs");
const input = fs
  .readFileSync("./day8")
  .toString()
  .split("\n")
  .filter((l) => l.length > 0)
  .map((l) => l.replace("+", " ").split(/\s+/));

const ops = {
  nop: (acc, arg, sptr) => ({ acc, sptr: sptr + 1 }),
  jmp: (acc, arg, sptr) => ({ acc, sptr: sptr + arg }),
  acc: (acc, arg, sptr) => ({ acc: acc + arg, sptr: sptr + 1 }),
};

const instructions = input;
const loopCheck = detectLoops(instructions, ops);
console.log("pt1: ", { acc: loopCheck.acc });

if (loopCheck.hasLoop) {
  const acc = debug(instructions, ops, loopCheck);
  console.log("pt2: ", { acc: acc.acc });
}

function debug(instructions, ops, loopCheck) {
  let result = null;
  // brute force. Not exactly efficient :(
  for (let frame of loopCheck.trace) {
    const tempInstructions = JSON.parse(JSON.stringify(instructions));
    const op = tempInstructions[frame.sptr][0];
    if (op === "nop") {
      tempInstructions[frame.sptr][0] = "jmp";
    } else if (op === "jmp") {
      tempInstructions[frame.sptr][0] = "nop";
    }
    result = detectLoops(tempInstructions, ops);
    if (result.hasLoop == false) break;
  }

  return result;
}

function detectLoops(instructions, ops) {
  let isDone = false;
  let hasLoop = false;
  let acc = 0;
  let sptr = 0;
  let visited = new Set();
  let trace = [];

  while (!isDone) {
    if (sptr > instructions.length - 1 || sptr < 0) {
      isDone = true;
    } else {
      if (visited.has(sptr)) {
        isDone = true;
        hasLoop = true;
      } else {
        const [instruction, offset] = instructions[sptr];
        visited.add(sptr);
        trace.push({ sptr, acc, op: { instruction, offset } });
        ({ acc, sptr } = ops[instruction](acc, parseInt(offset), sptr));
      }
    }
  }

  return {
    hasLoop,
    acc,
    sptr,
    trace,
  };
}
