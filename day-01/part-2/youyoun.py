from tool.runners.python import SubmissionPy

import re


def to_int(digit):
    if digit == "one" or digit == "1":
        return 1
    elif digit == "two" or digit == "2":
        return 2
    elif digit == "three" or digit == "3":
        return 3
    elif digit == "four" or digit == "4":
        return 4
    elif digit == "five" or digit == "5":
        return 5
    elif digit == "six" or digit == "6":
        return 6
    elif digit == "seven" or digit == "7":
        return 7
    elif digit == "eight" or digit == "8":
        return 8
    elif digit == "nine" or digit == "9":
        return 9


class YouyounSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        sum_ = 0
        for line in s.splitlines():
            digits = re.findall(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))", line)
            sum_ += to_int(digits[0]) * 10 + to_int(digits[-1])
        return sum_


def test_youyoun():
    """
    Run `python -m pytest ./day-01/part-2/youyoun.py` to test the submission.
    """
    assert (
        YouyounSubmission().run(
            """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
""".strip()
        )
        == 281
    )
