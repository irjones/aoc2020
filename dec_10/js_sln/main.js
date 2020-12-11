const fs = require("fs");
const fileData = fs.readFileSync("/Users/ianjones/desktop/day10").toString();
const None = Symbol("NONE");

const testData = `
16
10
15
5
1
11
7
19
6
12
4
`;
function RBTree(rootVal) {
  this.root = new Node(rootVal);
  this.values = new Set();
  this.values.add(rootVal);
}

RBTree.prototype.insert = function (val) {
  if (this.values.has(val)) {
    return false; // don't insert something we already have.
  }
  this.root.insert(val);
  return true;
};

RBTree.prototype.rotateLeft = function () {
  // TODO
};

RBTree.prototype.rotateRight = function () {
  // TODO
};

function Node(val, color, left = None, right = None) {
  this.val = val;
  // children- can be up to 3
  this.left = left;
  this.right = right;
}

Node.prototype.hasChildren = function () {
  return this.left != None || this.right != None;
};

Node.prototype.getLeaves = function () {
  if (!this.hasChildren) {
    return 1;
  }
  let leaves = 0;
  if (this.left != None) {
    leaves += this.left.getLeaves();
  }
  if (this.right != None) {
    leaves += this.right.getLeaves();
  }
};

Node.prototype.insert = function (val) {
  if (val > this.val) {
    if (this.right != None) {
      this.right.insert(val);
    } else {
      this.right = new Node(val);
    }
  } else {
    if (this.left != None) {
      this.left.insert(val);
    } else {
      this.left = new Node(val);
    }
  }
};

const data = testData;
// parse and prep array with 0 and max + 3 entries
const input = [0].concat(
  data
    .split("\n")
    .filter((l) => l.length)
    .map((l) => parseInt(l))
    .sort((l, r) => (l > r ? 1 : -1))
);
const max = input[input.length - 1] + 3;
input.push(max);

const pt1 = getOneOrThreeDiffs(input);
console.log({ pt1 });
const pt2 = getArrangements(input);

function getOneOrThreeDiffs(input) {
  const oneDiffs = [];
  const threeDiffs = [];

  for (let i = 0; i < input.length; i++) {
    const currentAdapter = input[i];
    const nextAdapter = input.length > i + 1 ? input[i + 1] : null;
    if (!nextAdapter) {
      // we're on the last one
      continue;
    }
    const diff = nextAdapter - currentAdapter;
    if (diff === 1) {
      oneDiffs.push(diff);
    } else if (diff === 3) {
      threeDiffs.push(diff);
    } else {
      console.log("not sure what to do with a diff of ", diff);
    }
  }

  return {
    ones: oneDiffs.length,
    threes: threeDiffs.length,
    answer: oneDiffs.length * threeDiffs.length,
  };
}

function getArrangements(input) {
  // TODO
  // load into RB tree
  // get leaves
  // should work?
}
