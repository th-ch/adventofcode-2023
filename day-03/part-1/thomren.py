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

        symbols = set()
        for match in finditer(r"[^(\.\d\n)]", s):
            start, end = match.span()
            row = start // (n_cols + 1)
            col = start % (n_cols + 1)
            symbols.add((row, col))

        part_numbers = []
        for match in finditer(r"\d+", s):
            start, end = match.span()
            n = int(match.group())
            row = start // (n_cols + 1)
            col_start = start % (n_cols + 1)
            col_end = end % (n_cols + 1)
            for i, j in product(
                range(row - 1, row + 2), range(col_start - 1, col_end + 1)
            ):
                if (i, j) in symbols:
                    part_numbers.append(n)
                    break
        return sum(part_numbers)


def test_thomren():
    """
    Run `python -m pytest ./day-03/part-1/thomren.py` to test the submission.
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
        == 4361
    )
