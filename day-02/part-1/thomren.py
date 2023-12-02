import re
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        res = 0
        for line in s.splitlines():
            game, samples = line.split(": ")
            game_id = int(game[5:])
            if is_possible(samples):
                res += game_id
        return res

MAX_BY_COLOR = {
    "red": 12,
    "green": 13,
    "blue": 14
}

def is_possible(samples: str) -> bool:
    for sample in samples.split("; "):
        for color_sample in sample.split(", "):
            n, color = color_sample.split()
            n = int(n)
            if n > MAX_BY_COLOR[color]:
                return False
    return True

def test_thomren():
    """
    Run `python -m pytest ./day-02/part-1/thomren.py` to test the submission.
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
        == 8
    )
