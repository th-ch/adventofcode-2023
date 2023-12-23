/**
 * @param s puzzle input in string format
 * @returns solution flag
 */
const run = (s: string): number => {
  // Your code goes here
  let card: string,
    content: string,
    winningString: string,
    selfString: string,
    winnings: number[],
    match: number,
    cardId: number,
    number: number;

  let total = 0;

  const multipliers = new Map<number, number>();

  for (const line of s.trim().split("\n")) {
    [card, content] = line.split(/: +/);
    [winningString, selfString] = content.split(/ +\| +/);
    winnings = winningString.split(/ +/).map((x) => parseInt(x, 10));
    match = selfString
      .split(/ +/)
      .map((x) => parseInt(x, 10))
      .filter((x) => winnings.includes(x)).length;

    cardId = parseInt(card.substring(4), 10);
    number = (multipliers.get(cardId) || 0) + 1;
    total += number;
    if (!match) {
      continue;
    }

    for (let i = 1; i <= match; i++) {
      multipliers.set(cardId + i, (multipliers.get(cardId + i) || 0) + number);
    }
  }

  return total;
};

console.assert(
  run(`Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11`) === 30
);

const start = performance.now();
const answer = run(Deno.args[0]);

console.log(`_duration:${performance.now() - start}`);
console.log(answer);
