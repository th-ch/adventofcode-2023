from tool.runners.python import SubmissionPy


def get_hash(step: str):
    val = 0
    for c in step:
        val += ord(c)
        val *= 17
        val %= 256
    return val


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        steps = s.split(",")
        return sum(get_hash(step) for step in steps)


def test_th_ch():
    """
    Run `python -m pytest ./day-15/part-1/th-ch.py` to test the submission.
    """
    assert get_hash("HASH") == 52
    assert (
        ThChSubmission().run(
            """
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
""".strip()
        )
        == 1320
    )
