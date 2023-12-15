from math import ceil
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        pipes = s.splitlines()
        start = next(
            (x, y)
            for x, line in enumerate(pipes)
            for y, c in enumerate(line)
            if c == "S"
        )
        loop_length = 1
        current = start
        dx, dy = 0, 0
        while current != start or loop_length == 1:
            (x, y) = current
            c = pipes[x][y]
            if c == "S":
                c = find_start_type(pipes, current)
            if c == "F":
                dx, dy = (0, 1) if dx == -1 else (1, 0)
            elif c == "7":
                dx, dy = (1, 0) if dy == 1 else (0, -1)
            elif c == "J":
                dx, dy = (0, -1) if dx == 1 else (-1, 0)
            elif c == "L":
                dx, dy = (-1, 0) if dy == -1 else (0, 1)
            elif c == "|":
                dx, dy = (1, 0) if dx == 0 else (dx, dy)
            elif c == "-":
                dx, dy = (0, 1) if dy == 0 else (dx, dy)
            else:
                raise ValueError(f"invalid pipe: {c} at {current}")

            current = (x + dx, y + dy)
            loop_length += 1

        return loop_length // 2


def find_start_type(pipes, start):
    (x, y) = start
    conn_north = pipes[x - 1][y] in ["F", "7", "|"]
    conn_south = pipes[x + 1][y] in ["L", "J", "|"]
    conn_west = pipes[x][y - 1] in ["-", "L", "F"]
    conn_east = pipes[x][y + 1] in ["-", "J", "7"]
    if conn_north and conn_east:
        return "L"
    elif conn_north and conn_west:
        return "J"
    elif conn_south and conn_east:
        return "F"
    elif conn_south and conn_west:
        return "7"
    elif conn_north and conn_south:
        return "|"
    elif conn_east and conn_west:
        return "-"
    else:
        raise ValueError("start must be a corner")


def test_thomren():
    """
    Run `python -m pytest ./day-10/part-1/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """
.....
.S-7.
.|.|.
.L-J.
.....
""".strip()
        )
        == 4
    )
    assert (
        ThomrenSubmission().run(
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
