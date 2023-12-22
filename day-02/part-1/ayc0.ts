const MAX_RED = 12;
const MAX_GREEN = 13;
const MAX_BLUE = 14;

/**
 * @param s puzzle input in string format
 * @returns solution flag
 */
const run = (s: string): number => {
  let score = 0;
  let red = 0;
  let blue = 0;
  let green = 0;

  let line: string,
    lineSegments: string[],
    pick: string[],
    pickScores: string,
    pickScore: string;

  loop: for (line of s.trim().split("\n")) {
    lineSegments = line.split(": ");

    for (pickScores of lineSegments[1].split("; ")) {
      for (pickScore of pickScores.split(", ")) {
        red = 0;
        blue = 0;
        green = 0;

        pick = pickScore.split(" ");

        if (pick[1] === "red") {
          red = parseInt(pick[0], 10);
          if (red > MAX_RED) {
            continue loop;
          }
        }
        if (pick[1] === "green") {
          green = parseInt(pick[0], 10);
          if (green > MAX_GREEN) {
            continue loop;
          }
        }
        if (pick[1] === "blue") {
          blue = parseInt(pick[0], 10);
          if (blue > MAX_BLUE) {
            continue loop;
          }
        }
      }
    }

    score += parseInt(lineSegments[0].substring(5), 10);
  }

  return score;
};

console.assert(
  run(`Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green`) === 8
);

const start = performance.now();
const answer = run(Deno.args[0]);

console.log(`_duration:${performance.now() - start}`);
console.log(answer);
