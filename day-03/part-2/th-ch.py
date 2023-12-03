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
            for x, c in enumerate(line):
                numbers = set()
                if c != "*":
                    continue
                # Found a gear
                # digit above?
                if y > 0:
                    for xx in [x - 1, x, x + 1]:
                        start, end = self.find_full_digit(lines[y - 1], xx)
                        if start is not None:
                            numbers.add((y - 1, start, end))
                # digit below?
                if y < len(lines) - 1:
                    for xx in [x - 1, x, x + 1]:
                        start, end = self.find_full_digit(lines[y + 1], xx)
                        if start is not None:
                            numbers.add((y + 1, start, end))
                # digit on the left?
                if x > 0:
                    start, end = self.find_full_digit(line[:x], x - 1)
                    if start is not None:
                        numbers.add((y, start, end))
                # digit on the right?
                if x < len(line) - 1:
                    start, end = self.find_full_digit(line[x + 1 :], 0)
                    if start is not None:
                        numbers.add((y, start + x + 1, end + x + 1))

                if len(numbers) == 2:
                    yy, x1, x2 = numbers.pop()
                    nb1 = int(lines[yy][x1 + 1 : x2])
                    yy, x1, x2 = numbers.pop()
                    nb2 = int(lines[yy][x1 + 1 : x2])
                    ratio = nb1 * nb2
                    result += ratio

        return result

    def find_full_digit(self, line, x):
        if not line[x].isdigit():
            return None, None

        start = x
        while start >= 0 and line[start].isdigit():
            start -= 1
        end = x
        while end < len(line) and line[end].isdigit():
            end += 1
        return start, end


def test_th_ch():
    """
    Run `python -m pytest ./day-03/part-2/th-ch.py` to test the submission.
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
        == 467835
    )
