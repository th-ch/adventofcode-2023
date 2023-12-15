import numpy as np
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        return sum(find_almost_symmetry(x) for x in s.split("\n\n"))


def find_almost_symmetry(s: str) -> int:
    a = np.array(
        [[1 if c == "#" else 0 for c in line] for line in s.splitlines()],
    )
    for i in range(1, len(a)):
        size = min(i, len(a) - i)
        x = a[i - size : i]
        y = a[i + size - 1 : i - 1 : -1]
        if np.sum(np.abs(x - y)) == 1:
            return 100 * i
    for j in range(1, a.shape[1]):
        size = min(j, len(a[0]) - j)
        x = a[:, j - size : j]
        y = a[:, j + size - 1 : j - 1 : -1]
        if np.sum(np.abs(x - y)) == 1:
            return j
    raise ValueError("no symmetry found")


def test_thomren():
    """
    Run `python -m pytest ./day-13/part-2/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
""".strip()
        )
        == 400
    )
