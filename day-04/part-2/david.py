from tool.runners.python import SubmissionPy


class DavidSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        pass


def test_david():
    """
    Run `python -m pytest ./day-04/part-2/david.py` to test the submission.
    """
    assert (
        DavidSubmission().run(
            """
""".strip()
        )
        == None
    )
