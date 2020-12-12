// day 12
const fs = require("fs");
const actualData = fs.readFileSync("../input").toString();
const testData = `
F10
N3
F7
R90
F11
`;

const input = actualData;
const DIRECTIONS = {
  North: "N",
  South: "S",
  East: "E",
  West: "W",
};

const TURNS = {
  Left: "L",
  Right: "R",
};

function Ship(initialCoords = { x: 0, y: 0 }, initialHeading = 90) {
  this.currentPosition = initialCoords;
  this.currentHeadingDegrees = initialHeading % 360;
  this.positionLog = [{ directive: "start", ...initialCoords }];
}

Ship.prototype.adjustHeading = function (modifier, amt) {
  if (modifier === TURNS.Left) {
    this.currentHeadingDegrees =
      (360 + (this.currentHeadingDegrees - amt)) % 360;
    return;
  }
  if (modifier === TURNS.Right) {
    this.currentHeadingDegrees = (this.currentHeadingDegrees + amt) % 360;
    return;
  }
};

Ship.prototype.getDirection = function () {
  if (this.currentHeadingDegrees in [0, 360]) {
    return DIRECTIONS.North;
  }
  if (this.currentHeadingDegrees === 90) {
    return DIRECTIONS.East;
  }
  if (this.currentHeadingDegrees === 180) {
    return DIRECTIONS.South;
  }
  if (this.currentHeadingDegrees === 270) {
    return DIRECTIONS.West;
  }
  console.log("Unexpected heading: " + this.currentHeadingDegrees);
};

Ship.prototype.move = function (instruction) {
  const directive =
    instruction[0] === "F" ? this.getDirection() : instruction[0];

  const val = parseInt(/\d+/.exec(instruction)[0]);

  if ([TURNS.Left, TURNS.Right].includes(directive)) {
    this.adjustHeading(directive, val);
    return;
  }
  if (directive === DIRECTIONS.North) {
    this.currentPosition.y += val;
  }
  if (directive === DIRECTIONS.East) {
    this.currentPosition.x += val;
  }
  if (directive === DIRECTIONS.South) {
    this.currentPosition.y -= val;
  }
  if (directive === DIRECTIONS.West) {
    this.currentPosition.x -= val;
  }
  this.positionLog.push({ directive, ...this.currentPosition });
};

function Course(directiveList, ship = new Ship()) {
  this.ship = ship;
  this.directives = directiveList.split("\n").filter((l) => l.length);
  this.path = [];
}

Course.prototype.navigate = function () {
  for (let directive of this.directives) {
    this.ship.move(directive);
  }
  console.log(
    "Pt1 - Distance from initial: ",
    Math.abs(this.ship.currentPosition.x) +
      Math.abs(this.ship.currentPosition.y)
  );
};

const c = new Course(input);
c.navigate();
