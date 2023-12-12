from tool.runners.python import SubmissionPy

from functools import cache


class RegexNotMatched(Exception):
    pass


def move_forward(remaining_in_group, i, truth, c):
    if c == ".":
        if remaining_in_group is not None:
            if remaining_in_group == 0:
                remaining_in_group = None
            else:
                raise RegexNotMatched()
    elif c == "#":
        if remaining_in_group is None:
            i += 1
            if i >= len(truth):
                raise RegexNotMatched()
            remaining_in_group = truth[i] - 1
        else:
            if remaining_in_group == 0:
                raise RegexNotMatched()
            else:
                remaining_in_group -= 1
    else:
        raise Exception("invalid char")

    return i, remaining_in_group

def ended(remaining_in_group, i, truth):
    return i == len(truth) - 1 and not remaining_in_group

def remaining(remaining_in_group, i, truth):
    remaining = sum(group + 1 for group in truth[i+1:]) - 1
    current = 0
    if remaining_in_group is not None:
        current = remaining_in_group + 1
    return remaining + current

@cache
def compute_nb(s: str, truth: tuple, i=-1, remaining_in_group=None):
    if not s:
        return int(ended(remaining_in_group, i, truth))

    if len(s) < remaining(remaining_in_group, i, truth):
        return 0

    if s[0] == "?":
        try:
            next_i, next_remaining_in_group = move_forward(remaining_in_group, i, truth, ".")
            dot = compute_nb(s[1:], truth, next_i, next_remaining_in_group)
        except RegexNotMatched:
            dot = 0

        try:
            next_i, next_remaining_in_group = move_forward(remaining_in_group, i, truth, "#")
            hashtag = compute_nb(s[1:], truth, next_i, next_remaining_in_group)
        except RegexNotMatched:
            hashtag = 0

        return dot + hashtag
    else:
        try:
            next_i, next_remaining_in_group = move_forward(remaining_in_group, i, truth, s[0])
        except RegexNotMatched:
            return 0

        return compute_nb(s[1:], truth, next_i, next_remaining_in_group)


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        result = 0
        for line in s.splitlines():
            unknown, truth = line.split()
            truth = tuple(int(t) for t in truth.split(","))

            nb = compute_nb(unknown, truth)
            result += nb
        return result



def test_th_ch():
    """
    Run `python -m pytest ./day-12/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
""".strip()
        )
        == 21
    )
