function checkNumber(str: string, at: number): number | undefined {
  let char = str[at];
  if (char >= "0" && char <= "9") {
    return parseInt(char, 10);
  }
  if (char === "o" && str[at + 1] === "n" && str[at + 2] === "e") {
    return 1;
  }
  if (char === "t" && str[at + 1] === "w" && str[at + 2] === "o") {
    return 2;
  }
  if (
    char === "t" &&
    str[at + 1] === "h" &&
    str[at + 2] === "r" &&
    str[at + 3] === "e" &&
    str[at + 4] === "e"
  ) {
    return 3;
  }
  if (
    char === "f" &&
    str[at + 1] === "o" &&
    str[at + 2] === "u" &&
    str[at + 3] === "r"
  ) {
    return 4;
  }
  if (
    char === "f" &&
    str[at + 1] === "i" &&
    str[at + 2] === "v" &&
    str[at + 3] === "e"
  ) {
    return 5;
  }
  if (char === "s" && str[at + 1] === "i" && str[at + 2] === "x") {
    return 6;
  }
  if (
    char === "s" &&
    str[at + 1] === "e" &&
    str[at + 2] === "v" &&
    str[at + 3] === "e" &&
    str[at + 4] === "n"
  ) {
    return 7;
  }
  if (
    char === "e" &&
    str[at + 1] === "i" &&
    str[at + 2] === "g" &&
    str[at + 3] === "h" &&
    str[at + 4] === "t"
  ) {
    return 8;
  }
  if (
    char === "n" &&
    str[at + 1] === "i" &&
    str[at + 2] === "n" &&
    str[at + 3] === "e"
  ) {
    return 9;
  }
  return undefined;
}

/**
 * @param s puzzle input in string format
 * @returns solution flag
 */
const run = (s: string): number => {
  const lines = s.trim().split("\n");
  let total = 0;
  let check;

  for (const line of lines) {
    for (let i = 0; i < line.length; i++) {
      check = checkNumber(line, i);
      if (check !== undefined) {
        total += check * 10;
        break;
      }
    }
    for (let i = line.length - 1; i >= 0; i--) {
      check = checkNumber(line, i);
      if (check !== undefined) {
        total += check;
        break;
      }
    }
  }

  return total;
};

console.assert(
  run(`two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen`) === 281
);

const start = performance.now();
const answer = run(Deno.args[0]);

console.log(`_duration:${performance.now() - start}`);
console.log(answer);
