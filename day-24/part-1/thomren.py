from dataclasses import dataclass

import numpy as np
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str, test_area=(200000000000000, 400000000000000)):
        """
        :param s: input in string format
        :return: solution flag
        """
        hailstones = []
        for line in s.splitlines():
            p, v = line.split(" @ ")
            p = Vec3(*map(int, p.split(",")))
            v = Vec3(*map(int, v.split(",")))
            hailstones.append(Hailstone(p, v))

        res = 0
        for i in range(len(hailstones)):
            # y = a1 * x + b1
            h1 = hailstones[i]
            a1 = h1.v.y / h1.v.x
            b1 = h1.p.y - a1 * h1.p.x
            for j in range(i + 1, len(hailstones)):
                # y = a2 * x + b2
                h2 = hailstones[j]
                a2 = h2.v.y / h2.v.x
                b2 = h2.p.y - a2 * h2.p.x

                # find intersection
                if abs(a1 - a2) < 1e-6:
                    # parallel lines, no intersection
                    continue
                ix = (b2 - b1) / (a1 - a2)
                iy = a1 * ix + b1
                t1 = (ix - h1.p.x) / h1.v.x
                t2 = (ix - h2.p.x) / h2.v.x

                if (
                    test_area[0] <= ix <= test_area[1]
                    and test_area[0] <= iy <= test_area[1]
                    and t1 >= 0
                    and t2 >= 0
                ):
                    res += 1
        return res


@dataclass
class Vec3:
    x: int
    y: int
    z: int


@dataclass
class Hailstone:
    p: Vec3
    v: Vec3


def test_thomren():
    """
    Run `python -m pytest ./day-24/part-1/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
""".strip(),
            test_area=(7, 27),
        )
        == 2
    )
