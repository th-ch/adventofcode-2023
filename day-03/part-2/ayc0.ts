function isNumber(char: string) {
  return char >= "0" && char <= "9";
}

function isGear(char: string) {
  return char === "*";
}

function findNumber(lines: string[], row: number, col: number) {
  let i = 0;
  while (isNumber(lines[row][col - i - 1])) {
    i++;
  }
  return parseInt(lines[row].substring(col - i), 10);
}

const offsets = [
  [-1, -1],
  [-1, 0],
  [-1, 1],
  [0, -1],
  [0, 1],
  [1, -1],
  [1, 0],
  [1, 1],
];

function checkGear(lines: string[], row: number, col: number) {
  const sides = new Set(
    offsets.filter((offset) =>
      isNumber(lines[row + offset[0]]?.[col + offset[1]])
    )
  );

  if (sides.has(offsets[1])) {
    sides.delete(offsets[0]);
    sides.delete(offsets[2]);
  }
  if (sides.has(offsets[6])) {
    sides.delete(offsets[5]);
    sides.delete(offsets[7]);
  }
  if (sides.size !== 2) {
    return false;
  }
  return Array.from(sides);
}

/**
 * @param s puzzle input in string format
 * @returns solution flag
 */
const run = (s: string): number => {
  // Your code goes here
  const lines = s.trim().split("\n");
  const nbOfRows = lines.length;
  const nbOfColumns = lines[0].length;

  let total = 0;

  let char: string;
  for (let row = 0; row < nbOfRows; row++) {
    for (let col = 0; col < nbOfColumns; col++) {
      char = lines[row][col];
      if (!isGear(char)) {
        continue;
      }
      const a = checkGear(lines, row, col);
      if (!a) {
        continue;
      }
      let prod = 1;
      prod *= findNumber(lines, row + a[0][0], col + a[0][1]);
      prod *= findNumber(lines, row + a[1][0], col + a[1][1]);
      total += prod;
    }
  }

  return total;
};

console.assert(
  run(`467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..`) === 467835
);

const start = performance.now();
const answer = run(Deno.args[0]);

console.log(`_duration:${performance.now() - start}`);
console.log(answer);
