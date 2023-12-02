from functools import reduce
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        res = 0 
        for line in s.splitlines():
            _, samples = line.split(": ")
            min_by_color = {
                "red": 0,
                "green": 0,
                "blue": 0
            }
            for sample in samples.split("; "):
                for color_sample in sample.split(", "):
                    n, color = color_sample.split()
                    n = int(n)
                    min_by_color[color] = max(min_by_color[color], n)
            res += reduce(lambda x, y: x * y, min_by_color.values(), 1)
        return res

def test_thomren():
    """
    Run `python -m pytest ./day-02/part-2/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
""".strip()
        )
        == 2286
    )
