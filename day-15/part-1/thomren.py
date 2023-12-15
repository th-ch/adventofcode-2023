from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        return sum(get_hash(x) for x in s.strip().split(","))


def get_hash(s: str) -> int:
    res = 0
    for c in s:
        res += ord(c)
        res *= 17
        res %= 256
    return res


def test_thomren():
    """
    Run `python -m pytest ./day-15/part-1/thomren.py` to test the submission.
    """
    assert get_hash("rn=1") == 30
    assert get_hash("cm-") == 253
    assert get_hash("qp=3") == 97
    assert get_hash("cm=2") == 47
    assert get_hash("qp-") == 14
    assert get_hash("pc=4") == 180
    assert get_hash("ot=9") == 9
    assert get_hash("ab=5") == 197
    assert get_hash("pc-") == 48
    assert get_hash("pc=6") == 214
    assert get_hash("ot=7") == 231
    assert (
        ThomrenSubmission().run("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
        == 1320
    )
