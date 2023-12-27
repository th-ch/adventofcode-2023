from tool.runners.python import SubmissionPy

from itertools import combinations

def line_intersection(line1, line2):
    xdiff = (line1[0][0] - line1[1][0], line2[0][0] - line2[1][0])
    ydiff = (line1[0][1] - line1[1][1], line2[0][1] - line2[1][1])

    def det(a, b):
        return a[0] * b[1] - a[1] * b[0]

    div = det(xdiff, ydiff)
    if div == 0:
       return None, None # no intersection

    d = (det(*line1), det(*line2))
    x = det(d, xdiff) / div
    y = det(d, ydiff) / div
    return x, y


class ThChSubmission(SubmissionPy):
    def run(self, s: str, intersect_min=200000000000000, intersect_max=400000000000000):
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
            paths.append(((px, py), (px+vx, py+vy)))

        crossing = 0
        for a, b in combinations(paths, 2):
            x, y = line_intersection(a, b)
            if x is None or y is None:
                continue
            if not (intersect_min <= x <= intersect_max and intersect_min <= y <= intersect_max):
                continue
            # Ensure collision is not in the past
            if (a[0][0] < a[1][0] <= x  or a[0][0] > a[1][0] >= x) and (a[0][1] < a[1][1] <= y or a[0][1] > a[1][1] >= y) and (b[0][0] < b[1][0] <= x  or b[0][0] > b[1][0] >= x) and (b[0][1] < b[1][1] <= y or b[0][1] > b[1][1] >= y):
                crossing += 1

        return crossing


def test_th_ch():
    """
    Run `python -m pytest ./day-24/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
""".strip(), intersect_min=7, intersect_max=27
        )
        == 2
    )
