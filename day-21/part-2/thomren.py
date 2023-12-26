from collections import defaultdict, deque
from math import ceil

import numpy as np
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str, n_steps: int = 26501365):
        grid = [[c for c in line] for line in s.splitlines()]
        height = len(grid)
        mod = n_steps % height
        distances = self.bfs(s, mod + 2 * height)
        y = [self.solve(distances, mod + i * height) for i in range(3)]
        p = np.poly1d(np.polyfit([0, 1, 2], y, 2))
        return round(p(n_steps // height))

    @staticmethod
    def solve(distances: dict[tuple[int, int], int], n_steps: int) -> int:
        return sum(
            1 for d in distances.values() if d % 2 == n_steps % 2 and d <= n_steps
        )

    @staticmethod
    def bfs(s: str, max_steps: int) -> dict[tuple[int, int], int]:
        grid = [[c for c in line] for line in s.splitlines()]
        start_idx = s.find("S")
        start = (start_idx // (len(grid) + 1), start_idx % (len(grid) + 1))
        queue = deque([(start, 0)])
        distances = {}
        while queue:
            (x, y), dist = queue.popleft()

            if dist > max_steps:
                break

            if (x, y) in distances:
                continue
            distances[(x, y)] = dist

            for dx, dy in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
                nx, ny = x + dx, y + dy
                if grid[nx % len(grid)][ny % len(grid[0])] != "#":
                    queue.append(((nx, ny), dist + 1))
        return distances


def test_thomren():
    """
    Run `python -m pytest ./day-21/part-2/thomren.py` to test the submission.
    """
    MAP = """
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
""".strip()
    solve = lambda n: ThomrenSubmission.solve(ThomrenSubmission.bfs(MAP, n), n)
    assert solve(6) == 16
    assert solve(10) == 50
    assert solve(50) == 1594
    assert solve(100) == 6536
    assert solve(500) == 167004
    assert solve(1000) == 668697
    # assert solve(5000) == 16733044
