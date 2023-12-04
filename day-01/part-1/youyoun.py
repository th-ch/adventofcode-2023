from tool.runners.python import SubmissionPy


class YouyounSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        sum_ = 0
        for line in s.splitlines():
            digits = [int(c) for c in line if c.isdigit()]
            sum_ += digits[0] * 10 + digits[-1]
        return sum_


def test_youyoun():
    """
    Run `python -m pytest ./day-01/part-1/youyoun.py` to test the submission.
    """
    assert (
        YouyounSubmission().run(
            """1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
""".strip()
        )
        == 142
    )
