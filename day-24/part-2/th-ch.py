from tool.runners.python import SubmissionPy

import numpy as np


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        lines = s.split("\n")
        paths = []
        for line in lines:
            point, vector = line.split(" @ ")
            px, py, pz = [int(i) for i in point.split(", ")]
            vx, vy, vz = [int(i) for i in vector.split(", ")]
            paths.append(((px, py, pz), (vx, vy, vz)))

        # Each hailstone h has trajectory p_h + t * v_h
        # The rock r has trajectory: p_r + t * v_r

        ####################

        # So for the first hailstone:
        # p1_x + t * v1_x = pr_x + t * vr_x
        # p1_y + t * v1_y = pr_x + t * vr_y
        # p1_z + t * v1_z = pr_x + t * vr_z

        ####################

        # With the first two equations
        # t = (p1_x - pr_x) / (vr_x - v1_x) and (p1_y - pr_y) / (vr_y - v1_y)
        # so (p1_x - pr_x) / (vr_x - v1_x) = (p1_y - pr_y) / (vr_y - v1_y)

        # p1_x * (vr_y - v1_y) - pr_x * (vr_y - v1_y) = p1_y * (vr_x - v1_x) - pr_y * (vr_x - v1_x)

        # p1_x * vr_y - p1_x * v1_y + pr_x * v1_y - p1_y * vr_x + p1_y * v1_x - pr_y * v1_x
        #    = pr_x * vr_y - pr_y * vr_x

        # The second part being constant, we can get the same with a second hailstone (2) and the rock (r)
        # p2_x * vr_y - p2_x * v2_y + pr_x * v2_y - p2_y * vr_x + p2_y * v2_x - pr_y * v2_x
        #   = pr_x * vr_y - pr_y * vr_x

        ####################

        # (v1_y - v2_y) * pr_x + (v2_x - v1_x) * pr_y + (p2_y - p1_y) * vr_x + (p1_x - p2_x) * vr_y
        #   = p1_x * v1_y - p1_y * v1_x - p2_x * v2_y + p2_y * v2_x
        # with 4 unknowns (pr_x, pr_y, vr_x, vr_y)

        # Doing the same for x and z gives another equation (y replaced by z) and 4 unknowns (pr_x, pr_z, vr_x, vr_z)
        # Then the same for y and z

        # So 3 equations for 6 unknowns, which means 3 paths are enough to solve the problem (linear: A * x = b)
        (p1_x, p1_y, p1_z), (v1_x, v1_y, v1_z) = paths[0]
        (p2_x, p2_y, p2_z), (v2_x, v2_y, v2_z) = paths[1]
        (p3_x, p3_y, p3_z), (v3_x, v3_y, v3_z) = paths[2]

        A = np.array([
            # Hailstones 1 and 2
            [v1_y - v2_y, v2_x - v1_x, 0, p2_y - p1_y, p1_x - p2_x, 0],
            [v1_z - v2_z, 0, v2_x - v1_x, p2_z - p1_z, 0, p1_x - p2_x],
            [0, v1_z - v2_z, v2_y - v1_y, 0, p2_z - p1_z, p1_y - p2_y],
            # Hailstones 2 and 3
            [v2_y - v3_y, v3_x - v2_x, 0, p3_y - p2_y, p2_x - p3_x, 0],
            [v2_z - v3_z, 0, v3_x - v2_x, p3_z - p2_z, 0, p2_x - p3_x],
            [0, v2_z - v3_z, v3_y - v2_y, 0, p3_z - p2_z, p2_y - p3_y],
        ])
        b = np.array([
            # Hailstones 1 and 2
            p1_x * v1_y - p1_y * v1_x - p2_x * v2_y + p2_y * v2_x,
            p1_x * v1_z - p1_z * v1_x - p2_x * v2_z + p2_z * v2_x,
            p1_y * v1_z - p1_z * v1_y - p2_y * v2_z + p2_z * v2_y,
            # Hailstones 2 and 3
            p2_x * v2_y - p2_y * v2_x - p3_x * v3_y + p3_y * v3_x,
            p2_x * v2_z - p2_z * v2_x - p3_x * v3_z + p3_z * v3_x,
            p2_y * v2_z - p2_z * v2_y - p3_y * v3_z + p3_z * v3_y,
        ])

        x = np.linalg.solve(A, b)
        # Keep only the first 3 values (pr_x, pr_y, pr_z) and sum them
        return x[:3].astype(int).sum()


def test_th_ch():
    """
    Run `python -m pytest ./day-24/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
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
