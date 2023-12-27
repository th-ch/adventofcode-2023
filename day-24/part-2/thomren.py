from dataclasses import dataclass
import numpy as np
from sympy import Matrix
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        hailstones = []
        for line in s.splitlines()[:3]:
            p, v = line.split(" @ ")
            p = np.array(list(map(int, p.split(","))))
            v = np.array(list(map(int, v.split(","))))
            hailstones.append(Hailstone(p, v))

        # p + v * t_i = p_i + v_i * t_i for i in [0,n]
        # ie (p - p_i) = t_i * (v_i - v)
        # ie (p - p_i) x (v_i - v) = 0 (colinear vectors)
        # substracting for 0 and 1 and 1 and 2
        # (v_0 - v_1) x p + (p_1 - p_0) x v = p_1 x v_1 - p_0 x v_0
        # (v_1 - v_2) x p + (p_2 - p_1) x v = p_2 x v_2 - p_1 x v_1

        A = np.zeros((6, 6), dtype=int)
        b = np.zeros(6, dtype=int)
        # using u x v = Uv where U = I x u
        A[:3, :3] = np.cross(np.eye(3), hailstones[0].v - hailstones[1].v)
        A[:3, 3:] = np.cross(np.eye(3), hailstones[1].p - hailstones[0].p)
        b[:3] = np.cross(hailstones[1].p, hailstones[1].v) - np.cross(
            hailstones[0].p, hailstones[0].v
        )
        A[3:6, :3] = np.cross(np.eye(3), hailstones[1].v - hailstones[2].v)
        A[3:6, 3:] = np.cross(np.eye(3), hailstones[2].p - hailstones[1].p)
        b[3:] = np.cross(hailstones[2].p, hailstones[2].v) - np.cross(
            hailstones[1].p, hailstones[1].v
        )

        x = Matrix(A).solve(Matrix(b))
        return sum(x[:3])


@dataclass
class Hailstone:
    p: np.ndarray
    v: np.ndarray


def test_thomren():
    """
    Run `python -m pytest ./day-24/part-2/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
""".strip()
        )
        == 47
    )
