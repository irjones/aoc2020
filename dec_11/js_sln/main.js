const fs = require("fs");
const actualData = fs.readFileSync("./day11").toString();

const None = "@";
const STATES = {
  FLOOR: ".",
  EMPTY_SEAT: "L",
  OCCUPIED_SEAT: "#",
};

function State(grid = []) {
  this.grid = grid;
}

State.prototype.equals = function (other) {
  return JSON.stringify(this.grid) === JSON.stringify(other.grid);
};

function tryGetFromArrOrNone(arr, y, x) {
  try {
    return arr[y][x] || None;
  } catch (err) {
    return None;
  }
}

State.prototype.toString = function () {
  return this.grid
    .map((y) => y.reduce((acc, next) => (acc += next), ""))
    .reduce((acc, next) => (acc += "\n" + next), "");
};

State.prototype.getNeighborsFor = function (x, y) {
  // clockwise starting at north
  return [
    [y - 1, x],
    [y - 1, x + 1],
    [y, x + 1],
    [y + 1, x + 1],
    [y + 1, x],
    [y + 1, x - 1],
    [y, x - 1],
    [y - 1, x - 1],
  ].map(([x, y]) => tryGetFromArrOrNone(this.grid, y, x));
};

State.prototype.getOccupiedSeats = function () {
  const asStr = this.toString();
  return asStr.split("").filter((c) => c === STATES.OCCUPIED_SEAT).length;
};

State.prototype.next = function () {
  const nextGrid = [];
  for (let y = 0; y < this.grid.length; y++) {
    const row = [];
    for (let x = 0; x < this.grid[y].length; x++) {
      let next = null;
      const current = this.grid[y][x];
      if (current === STATES.FLOOR) {
        next = STATES.FLOOR;
      }
      const neighbors = this.getNeighborsFor(y, x);

      if (current === STATES.EMPTY_SEAT) {
        if (neighbors.includes(STATES.OCCUPIED_SEAT)) {
          next = STATES.EMPTY_SEAT;
        } else {
          next = STATES.OCCUPIED_SEAT;
        }
      }
      if (current === STATES.OCCUPIED_SEAT) {
        if (neighbors.filter((n) => n === STATES.OCCUPIED_SEAT).length >= 4) {
          next = STATES.EMPTY_SEAT;
        } else {
          next = STATES.OCCUPIED_SEAT;
        }
      }
      row.push(next);
    }
    nextGrid.push(row);
  }
  return new State(nextGrid);
};

const data = actualData;

const initialState = new State(
  data
    .split("\n")
    .filter((l) => l.length > 0)
    .map((l) => l.split(""))
);

let count = 0;
let current = initialState.next();
let previous = initialState;

while (!current.equals(previous)) {
  previous = current;
  current = current.next();
  count += 1;
}

console.log(
  `Pt 1: After ${count} rounds, we've stabilized to ${current.getOccupiedSeats()} seats`
);

// TODO: pt 2