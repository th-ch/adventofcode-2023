from tool.runners.python import SubmissionPy


class YouyounSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        res = 0
        for line in s.splitlines():
            histories = [list(map(int, line.split()))]
            while not all(n == 0 for n in histories[-1]):
                histories.append(
                    [
                        histories[-1][i] - histories[-1][i - 1]
                        for i in range(1, len(histories[-1]))
                    ]
                )
            for i in range(len(histories) - 2, -1, -1):
                histories[i].insert(0, -histories[i + 1][0] + histories[i][0])

            res += histories[0][0]
        return res


def test_youyoun():
    """
    Run `python -m pytest ./day-09/part-2/youyoun.py` to test the submission.
    """
    assert (
        YouyounSubmission().run(
            """0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45""".strip()
        )
        == 2
    )
