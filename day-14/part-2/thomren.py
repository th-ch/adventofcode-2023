from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        rocks = set()
        cubes = set()
        lines = s.splitlines()
        for i, line in enumerate(lines):
            for j, c in enumerate(line):
                if c == "O":
                    rocks.add((i, j))
                elif c == "#":
                    cubes.add((i, j))

        seen = {}
        n_cycles = 1000000000
        it = 0
        while it < n_cycles:
            for di, dj in [(-1, 0), (0, -1), (1, 0), (0, 1)]:
                rocks_tmp = set()
                for i, j in sorted(rocks, key=lambda x: (-x[0] * di, -x[1] * dj)):
                    while (
                        i + di >= 0
                        and j + dj >= 0
                        and i + di < len(lines)
                        and j + dj < len(lines[0])
                        and (i + di, j + dj) not in rocks_tmp
                        and (i + di, j + dj) not in cubes
                    ):
                        i += di
                        j += dj
                    rocks_tmp.add((i, j))
                rocks = rocks_tmp

            state = tuple(sorted(rocks))
            if state in seen:
                cycle_length = it - seen[state]
                it += ((n_cycles - it) // cycle_length) * cycle_length
            else:
                seen[state] = it

            it += 1
        return sum(len(lines) - i for i, _ in rocks)


def test_thomren():
    """
    Run `python -m pytest ./day-14/part-2/thomren.py` to test the submission.
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
        == 64
    )
