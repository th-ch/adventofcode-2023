from tool.runners.python import SubmissionPy

import regex as re

class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        result = 0
        for line in s.splitlines():
            digits = re.findall(r"(\d|one|two|three|four|five|six|seven|eight|nine)", line, overlapped=True)
            result += _convert_to_int(digits[0])*10 + _convert_to_int(digits[-1])
        return result

def _convert_to_int(digit):
    if digit.isdigit():
        return int(digit)
    elif digit == "one":
        return 1
    elif digit == "two":
        return 2
    elif digit == "three":
        return 3
    elif digit == "four":
        return 4
    elif digit == "five":
        return 5
    elif digit == "six":
        return 6
    elif digit == "seven":
        return 7
    elif digit == "eight":
        return 8
    elif digit == "nine":
        return 9

def test_th_ch():
    """
    Run `python -m pytest ./day-01/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
two1nine
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
