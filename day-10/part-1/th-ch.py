from tool.runners.python import SubmissionPy

from math import inf
from queue import PriorityQueue


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        grid = [[c for c in line] for line in s.splitlines()]
        y_s = None
        for y, line in enumerate(grid):
            for x_s, c in enumerate(line):
                if c == "S":
                    y_s = y
                    break
            if y_s is not None:
                break

        scores = [[inf for _ in line] for line in grid]
        scores[y_s][x_s] = 0

        points = PriorityQueue()
        for y in range(len(grid)):
            for x in range(len(grid[y])):
                if grid[y][x] != ".":
                    points.put((scores[y][x], (x, y)))

        while not points.empty():
            _, (x, y) = points.get()
            next_points = []
            # left
            if x>0 and grid[y][x] in ["S", "-", "J", "7"] and grid[y][x-1] in ["-", "L", "F"]:
                next_points.append((x-1, y))
            # right
            if x<len(grid[y])-1 and grid[y][x] in ["S", "-", "L", "F"] and grid[y][x+1] in ["-", "J", "7"]:
                next_points.append((x+1, y))
            # down
            if y<len(grid)-1 and grid[y][x] in ["S", "|", "7", "F"] and grid[y+1][x] in ["|", "L", "J"]:
                next_points.append((x, y+1))
            # up
            if y>0 and grid[y][x] in ["S", "|", "L", "J"] and grid[y-1][x] in ["|", "7", "F"]:
                next_points.append((x, y-1))

            for xx, yy in next_points:
                alt = scores[y][x] + 1
                if alt < scores[yy][xx]:
                    scores[yy][xx] = alt
                    points.put((alt, (xx, yy)))

        return max(max(d if d != inf else -inf for d in line) for line in scores)


def test_th_ch():
    """
    Run `python -m pytest ./day-10/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
""".strip()
        )
        == 8
    )
