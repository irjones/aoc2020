const fs = require("fs");
const testRules = `
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
`
  .split("\n")
  .filter((l) => l.length !== 0);

const data = fs
  .readFileSync("/Users/ianjones/Desktop/day7")
  .toString()
  .split("\n")
  .filter((l) => l.length !== 0);

function Node(val, edges = []) {
  this.val = val;
  this.edges = edges;
}

Node.prototype.addEdge = function (node) {
  this.edges.push(node);
};

Node.prototype.has = function (v) {
  if (this.val === v) return true;
  // well, no luck there. better do a search.
  let queue = [...this.edges];
  while (queue.length > 0) {
    const interval = queue.length;
    for (let i = 0; i < interval; i++) {
      const current = queue.shift();
      if (current.val === v) {
        return true;
      }
      queue = queue.concat(current.edges);
    }
  }
  return false; // wasn't in there.
};

const filterLabel = (namePart) => {
  const pattern = RegExp("[^\\d\\s]+");
  return pattern.test(namePart) && !namePart.includes("bag");
};

const toLabel = (containerName) => {
  return containerName.split(" ").filter(filterLabel).join("");
};

const parseRule = (line) => {
  const [container, contents] = line.split("contain");
  const expandedContents = contents.split(",");
  const map = new Map();
  return {
    label: toLabel(container),
    expandedContents: expandedContents.map(toLabel),
  };
};

const makeNodes = (rules) => {
  const nodes = {};
  for (let rule of rules) {
    if (!nodes[rule.label]) {
      nodes[rule.label] = new Node(rule.label);
    }
  }

  for (let rule of rules) {
    for (let edge of rule.expandedContents) {
      if (edge !== "noother") {
        nodes[rule.label].addEdge(nodes[edge]);
      }
    }
  }

  return nodes;
};

const parsedRules = data.map(parseRule);
const nodeList = makeNodes(parsedRules);
delete nodeList["shinygold"]; // don't check the shiny gold bag for a shiny gold bag, it inflates by 1
const pt1 = Object.values(nodeList)
  .map((node) => node.has("shinygold"))
  .filter((a) => a).length;
console.log(`PT1 - ${pt1}`);