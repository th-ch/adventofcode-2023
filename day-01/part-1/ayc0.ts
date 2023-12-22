/**
 * @param s puzzle input in string format
 * @returns solution flag
 */
const run = (s: string): number => {
  const lines = s.trim().split("\n");
  let total = 0;
  let char;
  for (const line of lines) {
    for (let i = 0; i < line.length; i++) {
      char = line[i];
      if (char === "0") {
        break;
      }
      if (char >= "1" && char <= "9") {
        total += parseInt(char + "0", 10);
        break;
      }
    }
    for (let i = line.length - 1; i >= 0; i--) {
      char = line[i];
      if (char === "0") {
        break;
      }
      if (char >= "1" && char <= "9") {
        total += parseInt(char, 10);
        break;
      }
    }
  }

  return total;
};

console.assert(
  run(`1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet`) === 142
);

const start = performance.now();
const answer = run(Deno.args[0]);

console.log(`_duration:${performance.now() - start}`);
console.log(answer);
