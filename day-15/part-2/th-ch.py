from tool.runners.python import SubmissionPy
from importlib import import_module

from collections import defaultdict, OrderedDict

part1 = import_module("day-15.part-1.th-ch")


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        steps = s.split(",")
        boxes = defaultdict(OrderedDict)
        for step in steps:
            operator = "=" if "=" in step else "-"
            label, focal_length = step.split(operator)
            box = part1.get_hash(label)
            if operator == "-":
                boxes[box].pop(label, None)
            else:
                boxes[box][label] = int(focal_length)

        power = 0
        for box, lens in boxes.items():
            for i, focal_length in enumerate(lens.values()):
                power += (box+1) * (i+1) * focal_length
        return power

def test_th_ch():
    """
    Run `python -m pytest ./day-15/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
""".strip()
        )
        == 145
    )
