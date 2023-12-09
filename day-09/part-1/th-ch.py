from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        result = 0
        for history in s.splitlines():
            numbers = [int(n) for n in history.split()]
            last_digits = [numbers[-1]]
            while True:
                diffs = [numbers[i] - numbers[i - 1] for i in range(1, len(numbers))]
                if all(d == 0 for d in diffs):
                    break
                else:
                    numbers = diffs
                    last_digits.append(diffs[-1])
            result += sum(last_digits)
        return result

def test_th_ch():
    """
    Run `python -m pytest ./day-09/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
""".strip()
        )
        == 114
    )
