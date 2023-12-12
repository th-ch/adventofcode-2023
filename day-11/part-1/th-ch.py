from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        galaxies = set()
        for y, line in enumerate(s.splitlines()):
            for x, c in enumerate(line):
                if c == "#":
                    galaxies.add((x, y))

        # Expand galaxies
        w, h = x, y
        empty_columns = [x for x in range(w) if x not in set(x for x, y in galaxies)]
        empty_rows = [y for y in range(h) if y not in set(y for x, y in galaxies)]
        expanded_galaxies = set()
        for x, y in galaxies:
            offset_x = len([empty_x for empty_x in empty_columns if empty_x < x])
            offset_y = len([empty_y for empty_y in empty_rows if empty_y < y])
            expanded_galaxies.add((x + offset_x, y + offset_y))

        # Distances
        result = 0
        for g1 in expanded_galaxies:
            for g2 in expanded_galaxies:
                if g1 == g2:
                    continue
                dist = abs(g1[0] - g2[0]) + abs(g1[1] - g2[1])
                result += dist
        return result // 2

def test_th_ch():
    """
    Run `python -m pytest ./day-11/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
""".strip()
        )
        == 374
    )
