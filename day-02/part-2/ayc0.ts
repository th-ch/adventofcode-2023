/**
 * @param s puzzle input in string format
 * @returns solution flag
 */
const run = (s: string): number => {
  let score = 0,
    red: number,
    blue: number,
    green: number,
    minRed: number,
    minBlue: number,
    minGreen: number;

  let line: string,
    lineSegments: string[],
    pick: string[],
    pickScores: string,
    pickScore: string;

  for (line of s.trim().split("\n")) {
    minRed = 1;
    minBlue = 1;
    minGreen = 1;

    lineSegments = line.split(": ");

    for (pickScores of lineSegments[1].split("; ")) {
      for (pickScore of pickScores.split(", ")) {
        red = 0;
        blue = 0;
        green = 0;

        pick = pickScore.split(" ");

        if (pick[1] === "red") {
          red = parseInt(pick[0], 10);
        }
        if (pick[1] === "green") {
          green = parseInt(pick[0], 10);
        }
        if (pick[1] === "blue") {
          blue = parseInt(pick[0], 10);
        }

        if (red > minRed) {
          minRed = red;
        }
        if (green > minGreen) {
          minGreen = green;
        }
        if (blue > minBlue) {
          minBlue = blue;
        }
      }
    }

    score += minRed * minGreen * minBlue;
  }

  return score;
};

console.assert(
  run(
    `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green`
  ) === 2286
);

const start = performance.now();
const answer = run(Deno.args[0]);

console.log(`_duration:${performance.now() - start}`);
console.log(answer);
