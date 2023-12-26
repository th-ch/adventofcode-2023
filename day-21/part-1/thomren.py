from collections import deque
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str, n_steps: int = 64):
        """
        :param s: input in string format
        :return: solution flag
        """
        grid = [[c for c in line] for line in s.splitlines()]
        start_idx = s.find("S")
        start = (start_idx // (len(grid) + 1), start_idx % (len(grid) + 1))

        res = 0
        queue = deque([(start, 0)])
        visited = {}
        while queue:
            (x, y), dist = queue.popleft()

            if (x, y) in visited:
                continue
            visited[(x, y)] = dist

            if dist > n_steps:
                continue

            for dx, dy in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
                nx, ny = x + dx, y + dy
                if (
                    0 <= nx < len(grid)
                    and 0 <= ny < len(grid[0])
                    and grid[nx][ny] != "#"
                ):
                    queue.append(((nx, ny), dist + 1))
        return sum(1 for d in visited.values() if d % 2 == n_steps % 2)


def test_thomren():
    """
    Run `python -m pytest ./day-21/part-1/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """
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
""".strip(),
            6,
        )
        == 16
    )
