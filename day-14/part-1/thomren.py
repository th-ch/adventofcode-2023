from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        rocks_start = set()
        cubes = set()
        lines = s.splitlines()
        for i, line in enumerate(lines):
            for j, c in enumerate(line):
                if c == "O":
                    rocks_start.add((i, j))
                elif c == "#":
                    cubes.add((i, j))

        rocks_end = set()
        for i, j in sorted(rocks_start):
            while i >= 1 and (i - 1, j) not in rocks_end and (i - 1, j) not in cubes:
                i -= 1
            rocks_end.add((i, j))
        return sum(len(lines) - i for i, _ in rocks_end)


def test_thomren():
    """
    Run `python -m pytest ./day-14/part-1/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
""".strip()
        )
        == 136
    )
