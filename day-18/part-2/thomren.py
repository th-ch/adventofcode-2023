from tool.runners.python import SubmissionPy

DIRECTIONS = [(0, 1), (1, 0), (0, -1), (-1, 0)]


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        area = 0
        length = 0
        x, y = 0, 0
        for line in s.splitlines():
            code = line.split()[-1]
            distance = int(code[2:7], 16)
            dx, dy = DIRECTIONS[int(code[-2])]

            x += dx * distance
            y += dy * distance
            length += distance
            # Green's theroem
            # ∫_{C}(Ldx + Mdy)=∫∫_{D}(dM/dx - dL/dy)dxdy
            # with L = 0, M = x, we have
            # ∫xdy=∫∫dxdy
            area += x * dy * distance
        return abs(area) + length // 2 + 1


def test_thomren():
    """
    Run `python -m pytest ./day-18/part-1/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
""".strip()
        )
        == 952408144115
    )
