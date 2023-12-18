from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        edge = [[0, 0]]
        boundary = 0
        for dig in s.splitlines():
            _, _, color = dig.split()
            color = color[2:-1]
            nb = int(color[:5], 16)
            direction = ["R", "D", "L", "U"][int(color[5:])]

            x, y = edge[-1]
            if direction == "R":
                x, y = x+nb, y
            elif direction == "L":
                x, y = x-nb, y
            elif direction == "U":
                x, y = x, y-nb
            elif direction == "D":
                x, y = x, y+nb
            else:
                raise ValueError("Unknown direction")
            edge.append([x, y])
            boundary += nb

        # https://stackoverflow.com/questions/16342200/how-can-i-calculate-the-area-of-an-object-by-using-its-contour-chain-code
        area = (sum(edge[i][0] * edge[i+1][1] - edge[i+1][0] * edge[i][1] for i in range(len(edge)-1)) + boundary + 2) // 2
        return area


def test_th_ch():
    """
    Run `python -m pytest ./day-18/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
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
