from tool.runners.python import SubmissionPy

from collections import defaultdict, deque
from importlib import import_module

part1 = import_module("day-16.part-1.th-ch")


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        grid = defaultdict(lambda: defaultdict(lambda: "."))
        for y, line in enumerate(s.splitlines()):
            for x, char in enumerate(line):
                if char == ".":
                    continue
                grid[y][x] = char
        w, h = x + 1 , y + 1

        max_energized = 0
        for y in range(h):
            energized = part1.count_energized(grid, w, h, (0, y, part1.RIGHT))
            max_energized = max(max_energized, energized)
            energized = part1.count_energized(grid, w, h, (w-1, y, part1.LEFT))
            max_energized = max(max_energized, energized)
        for x in range(w):
            energized = part1.count_energized(grid, w, h, (x, 0, part1.DOWN))
            max_energized = max(max_energized, energized)
            energized = part1.count_energized(grid, w, h, (x, h-1, part1.UP))
            max_energized = max(max_energized, energized)

        return max_energized


def test_th_ch():
    """
    Run `python -m pytest ./day-16/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
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
