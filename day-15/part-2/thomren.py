from collections import OrderedDict, defaultdict
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        boxes = [OrderedDict() for _ in range(256)]
        for ins in s.strip().split(","):
            if ins[-1] == "-":
                label = ins[:-1]
                box = boxes[get_hash(label)]
                box.pop(label, None)
            else:
                label, focal_length = ins.split("=")
                box = boxes[get_hash(label)]
                focal_length = int(focal_length)
                box[label] = focal_length

        res = 0
        for i, box in enumerate(boxes):
            for j, focal_length in enumerate(box.values()):
                res += (i + 1) * (j + 1) * focal_length
        return res


def get_hash(s: str) -> int:
    res = 0
    for c in s:
        res += ord(c)
        res *= 17
        res %= 256
    return res


def test_thomren():
    """
    Run `python -m pytest ./day-15/part-2/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
        == 145
    )
