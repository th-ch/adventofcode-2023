from math import ceil, floor, sqrt
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str) -> int:
        """
        :param s: input in string format
        :return: solution flag
        """
        lines = s.splitlines()
        times = [int(n) for n in lines[0].split()[1:]]
        distances = [int(n) for n in lines[1].split()[1:]]

        combinations = 1
        for (time, distance) in zip(times, distances):
            mini, maxi = self.solve_race(time, distance)
            combinations *= maxi - mini + 1
        return combinations

    @staticmethod
    def solve_race(time: int, distance: int) -> (int, int):
        """ Return the min and max x values such that (time - x) * x > distance """
        # x**2 - x * time + distance < 0
        delta = time**2 - 4 * distance
        roots = (time - sqrt(delta)) / 2, (time + sqrt(delta)) / 2
        mini = int(roots[0] + 1) if roots[0].is_integer() else ceil(roots[0])
        maxi = int(roots[1] - 1) if roots[1].is_integer() else floor(roots[1])
        return mini, maxi


def test_thomren():
    """
    Run `python -m pytest ./day-06/part-1/thomren.py` to test the submission.
    """
    assert ThomrenSubmission().solve_race(7, 9) == (2, 5)
    assert ThomrenSubmission().solve_race(15, 40) == (4, 11)
    assert ThomrenSubmission().solve_race(30, 200) == (11, 19)
    assert (
        ThomrenSubmission().run(
            """
Time:      7  15   30
Distance:  9  40  200
""".strip()
        )
        == 288
    )
