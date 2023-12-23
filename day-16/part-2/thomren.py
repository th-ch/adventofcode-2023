from collections import defaultdict
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        grid = [[c for c in line] for line in s.split("\n")]
        res = 0
        for x in range(len(grid)):
            res = max(res, energize(grid, x, 0, 0, 1))
            res = max(res, energize(grid, x, len(grid[0]) - 1, 0, -1))
        for y in range(len(grid[0])):
            res = max(res, energize(grid, 0, y, 1, 0))
            res = max(res, energize(grid, len(grid) - 1, y, -1, 0))
        return res


def energize(grid, x, y, dx, dy):
    visited = defaultdict(set)
    stack = [(x, y, dx, dy)]

    while len(stack) > 0:
        (x, y, dx, dy) = stack.pop()
        if (
            x < 0
            or x >= len(grid)
            or y < 0
            or y >= len(grid[0])
            or ((x, y) in visited and (dx, dy) in visited[(x, y)])
        ):
            continue
        visited[(x, y)].add((dx, dy))
        if grid[x][y] == ".":
            stack.append((x + dx, y + dy, dx, dy))
        elif grid[x][y] == "/":
            dx, dy = -dy, -dx
            stack.append((x + dx, y + dy, dx, dy))
        elif grid[x][y] == "\\":
            dx, dy = dy, dx
            stack.append((x + dx, y + dy, dx, dy))
        elif grid[x][y] == "-":
            if dx == 0:
                stack.append((x + dx, y + dy, dx, dy))
            else:
                stack.append((x, y + 1, 0, 1))
                stack.append((x, y - 1, 0, -1))
        elif grid[x][y] == "|":
            if dy == 0:
                stack.append((x + dx, y + dy, dx, dy))
            else:
                stack.append((x + 1, y, 1, 0))
                stack.append((x - 1, y, -1, 0))

    return len(visited)


def test_thomren():
    """
    Run `python -m pytest ./day-16/part-2/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            r"""
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
""".strip()
        )
        == 51
    )
