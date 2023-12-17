from tool.runners.python import SubmissionPy

def move_north(rocks, blocks, w, h):
    for y in range(h):
        rocks_by_y = set(xx for xx, yy in rocks if yy == y)
        for x in rocks_by_y:
            blocks_before = [yy for xx, yy in blocks if xx == x and yy < y]
            new_y = max(blocks_before) + 1 if blocks_before else 0
            while new_y < y and (x, new_y) in rocks:
                new_y += 1
            rocks.remove((x, y))
            rocks.add((x, new_y))

    return rocks


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        rocks = set()
        blocks = set()
        for y, line in enumerate(s.splitlines()):
            for x, c in enumerate(line):
                if c == "O":
                    rocks.add((x, y))
                elif c == "#":
                    blocks.add((x, y))

        w, h = x + 1, y + 1

        rocks = move_north(rocks, blocks, w, h)

        return sum(sum(1 for _, yy in rocks if yy == y) * (h-y) for y in range(h))

def print_platform(rocks, blocks, w, h):
    for y in range(h):
        line = ""
        for x in range(w+1):
            if (x,y) in rocks:
                line += "O"
            elif (x, y) in blocks:
                line += "#"
            else:
                line += "."
        print(line)


def test_th_ch():
    """
    Run `python -m pytest ./day-14/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...
""".strip()
        )
        == 136
    )
