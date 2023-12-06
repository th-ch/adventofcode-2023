from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        lines = s.splitlines()
        times = [int(t) for t in lines[0][11:].split()]
        distances = [int(t) for t in lines[1][11:].split()]

        result = 1
        for i in range(len(times)):
            beating_record = 0
            for hold in range(times[i]):
                d = (times[i] - hold) * hold
                if d > distances[i]:
                    beating_record += 1
            result *= beating_record
        return result

def test_th_ch():
    """
    Run `python -m pytest ./day-06/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
Time:      7  15   30
Distance:  9  40  200
""".strip()
        )
        == 288
    )
