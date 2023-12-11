from itertools import product
import numpy as np
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str, expansion_factor=1000000):
        """
        :param s: input in string format
        :return: solution flag
        """
        grid = np.array(
            [[1 if c == "#" else 0 for c in line] for line in s.splitlines()]
        )
        y_offsets = ((grid.sum(axis=0) == 0)).cumsum() * (expansion_factor - 1)
        x_offsets = ((grid.sum(axis=1) == 0)).cumsum() * (expansion_factor - 1)
        xs, ys = np.nonzero(grid)
        xs += x_offsets[xs]
        ys += y_offsets[ys]
        galaxies = list(zip(xs, ys))

        sum_distances = 0
        for i in range(len(galaxies)):
            for j in range(i + 1, len(galaxies)):
                dist = abs(galaxies[i][0] - galaxies[j][0]) + abs(
                    galaxies[i][1] - galaxies[j][1]
                )
                sum_distances += dist
        return sum_distances


def test_thomren():
    """
    Run `python -m pytest ./day-11/part-2/thomren.py` to test the submission.
    """
    test_input = """
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
""".strip()

    assert ThomrenSubmission().run(test_input, 10) == 1030
    assert ThomrenSubmission().run(test_input, 100) == 8410
