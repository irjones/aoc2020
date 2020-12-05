// TODO: write this as Rust instead of JS

const fs = require("fs");
const lines = fs
    .readFileSync("/Users/ianjones/Desktop/day5")
    .toString()
    .split("\n")
    .filter((l) => l.length);

const results = new Set(
    lines
        .map((l) => l.split(""))
        .map(get_seat_coords)
        .map(getSeatId)
);

const min = Math.min(...results);
const max = Math.max(...results);

const expectedSet = new Set();
for (let i = min; i < max; i++) {
    expectedSet.add(i);
}

console.log("highest ID:", max);
const diff = Array.from(expectedSet.difference(results));
if (diff.length === 1) {
    console.log("My Seat:", diff[0]);
} else {
    console.error("It's all gone wrong. Multiple seats available?!", diff);
}

// helpers //

function upperHalf(range) {
    const { lower, upper } = range;
    return {
        lower: Math.ceil((upper + lower) / 2),
        upper,
    };
}

function lowerHalf(range) {
    const { lower, upper } = range;
    return {
        lower,
        upper: Math.floor((upper + lower) / 2),
    };
}

function get_seat_coords(inputArr) {
    let rowPos = {
        lower: 0,
        upper: 127,
    };
    let columnPos = {
        lower: 0,
        upper: 7,
    };
    while (inputArr.length > 0) {
        const next = inputArr.shift();
        switch (next) {
            case "F": {
                rowPos = lowerHalf(rowPos);
                break;
            }
            case "B": {
                rowPos = upperHalf(rowPos);
                break;
            }
            case "R": {
                columnPos = upperHalf(columnPos);
                break;
            }
            case "L": {
                columnPos = lowerHalf(columnPos);
                break;
            }
            default: {
                console.warn("NOP for 'next' value of " + next);
            }
        }
    }

    if (columnPos.upper === columnPos.lower && rowPos.upper === rowPos.lower) {
        return {
        column: columnPos.upper,
        row: rowPos.upper,
        };
    } else {
        console.error("failed to settle on solution for input " + inputs);
    }
}

function getSeatId(seatCoords) {
    const { column, row } = seatCoords;
    return row * 8 + column;
}
