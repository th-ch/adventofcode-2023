from tool.runners.python import SubmissionPy

class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        result = 0
        for line in s.splitlines():
            digits = [int(c) for  c in line if c.isdigit()]
            result += digits[0]*10 + digits[-1]

        return result


def test_th_ch():
    """
    Run `python -m pytest ./day-01/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
""".strip()
        )
        == 142
    )
