function isNumber(char: string) {
  return char >= "0" && char <= "9";
}
function isSymbol(char: string | undefined) {
  return char !== undefined && !isNumber(char) && char !== "." && char;
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

function checkSides(lines: string[], row: number, col: number) {
  return offsets.some((offset) =>
    isSymbol(lines[row + offset[0]]?.[col + offset[1]])
  );
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
      if (!isNumber(char)) {
        continue;
      }
      let i = 0;
      let keep = false;
      while (true) {
        if (checkSides(lines, row, col + i)) {
          keep = true;
          break;
        }
        if (!isNumber(lines[row][col + ++i])) {
          break;
        }
      }

      const curNumber = parseInt(lines[row].substr(col), 10);
      if (keep) {
        total += curNumber;
      }
      col += String(curNumber).length;
      continue;
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
.664.598..`) === 4361
);

const start = performance.now();
const answer = run(Deno.args[0]);

console.log(`_duration:${performance.now() - start}`);
console.log(answer);
