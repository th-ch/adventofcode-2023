from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        result = 0
        lines = s.splitlines()
        for y, line in enumerate(lines):
            start, end = None, None
            for x, c in enumerate(line + "."):
                if c.isdigit():
                    if start is None:
                        start = x
                elif start is not None:
                    # Full digit
                    end = x
                    neighbors = (
                        [(i, y - 1) for i in range(start - 1, end + 1)]
                        + [(start - 1, y), (end, y)]
                        + [(i, y + 1) for i in range(start - 1, end + 1)]
                    )
                    for n_x, n_y in neighbors:
                        try:
                            n = lines[n_y][n_x]
                        except IndexError:
                            continue
                        is_part_nb = not n.isdigit() and n != "."
                        if is_part_nb:
                            break
                    if is_part_nb:
                        result += int(line[start:end])

                    # reinit
                    start = None
                    end = None
        return result


def test_th_ch():
    """
    Run `python -m pytest ./day-03/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
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
