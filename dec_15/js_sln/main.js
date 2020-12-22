const testNums = [0, 3, 6];
const actualNums = [14, 8, 16, 0, 1, 17];

const startingNums = actualNums;
const history = new History(startingNums);
for (let i = 0; i < 30000000 - actualNums.length; i++) {
  history.next();
}
const pt1 = history.turnLog[2020 - 1];
const pt2 = history.turnLog[30000000 - 1];
console.log({
  pt1,
  pt2,
});

function History(seed) {
  this.seen = new Map();
  this.turnLog = [];
  this.turn = seed.length;

  for (let i = 0; i < seed.length; i++) {
    this.seen.set(seed[i], { ultimate: i + 1, penultimate: 0 });
    this.turnLog.push({ turn: i + 1, num: seed[i] });
  }

  this.next = function () {
    const { num } = this.turnLog[this.turn - 1];
    const { ultimate, penultimate } = this.seen.get(num);
    this.turn += 1;
    const nextEntry = {
      turn: this.turn,
      num: penultimate === 0 ? 0 : ultimate - penultimate,
    };
    this.seen.set(nextEntry.num, {
      ultimate: this.turn,
      penultimate: (this.seen.get(nextEntry.num) || { ultimate: 0 }).ultimate,
    });
    this.turnLog.push(nextEntry);
    return nextEntry;
  };
}
