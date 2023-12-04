from itertools import product
from re import finditer
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str) -> int:
        """
        :param s: input in string format
        :return: solution flag
        """
        n_cols = s.find("\n")

        part_numbers = {}
        part_uid = 0
        for match in finditer(r"\d+", s):
            part_uid += 1
            n = int(match.group())
            start, end = match.span()
            row = start // (n_cols + 1)
            col_start = start % (n_cols + 1)
            col_end = end % (n_cols + 1)
            for col in range(col_start, col_end):
                part_numbers[(row, col)] = (n, part_uid)

        res = 0
        for match in finditer(r"\*", s):
            start, end = match.span()
            row = start // (n_cols + 1)
            col = start % (n_cols + 1)
            adjacent_parts = {
                part_numbers[(row, col)]
                for row, col in product(
                    range(row - 1, row + 2), range(col - 1, col + 2)
                )
                if (row, col) in part_numbers
            }
            if len(adjacent_parts) == 2:
                res += list(adjacent_parts)[0][0] * list(adjacent_parts)[1][0]

        return res


def test_thomren():
    """
    Run `python -m pytest ./day-03/part-2/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
""".strip()
        )
        == 467835
    )
