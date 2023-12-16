from tool.runners.python import SubmissionPy

from importlib import import_module

part1 = import_module("day-14.part-1.th-ch")
move_north = part1.move_north

def move_west(rocks, blocks, w, h):
    for x in range(w):
        rocks_by_y = set(yy for xx, yy in rocks if xx == x)
        for y in rocks_by_y:
            blocks_before = [xx for xx, yy in blocks if yy == y and xx < x]
            new_x = max(blocks_before) + 1 if blocks_before else 0
            while new_x < x and (new_x, y) in rocks:
                new_x += 1
            rocks.remove((x, y))
            rocks.add((new_x, y))

    return rocks


def move_east(rocks, blocks, w, h):
    for x in range(w, -1, -1):
        rocks_by_y = set(yy for xx, yy in rocks if xx == x)
        for y in rocks_by_y:
            blocks_before = [xx for xx, yy in blocks if yy == y and xx > x]
            new_x = min(blocks_before) - 1 if blocks_before else w - 1
            while new_x > x and (new_x, y) in rocks:
                new_x -= 1
            rocks.remove((x, y))
            rocks.add((new_x, y))

    return rocks


def move_south(rocks, blocks, w, h):
    for y in range(h, -1, -1):
        rocks_by_y = set(xx for xx, yy in rocks if yy == y)
        for x in rocks_by_y:
            blocks_before = [yy for xx, yy in blocks if xx == x and yy > y]
            new_y = min(blocks_before) - 1 if blocks_before else h - 1
            while new_y > y and (x, new_y) in rocks:
                new_y -= 1
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

        states = {}
        nb_cycles = 1000000000
        loop_found = False
        i = 0
        while i < nb_cycles:
            rocks = move_north(rocks, blocks, w, h)
            rocks = move_west(rocks, blocks, w, h)
            rocks = move_south(rocks, blocks, w, h)
            rocks = move_east(rocks, blocks, w, h)
            if loop_found:
                i += 1
                continue

            new_state = tuple(sorted(rocks))
            if new_state in states:
                loop = i - states[new_state]
                i += ((nb_cycles - i) // loop) * loop
                loop_found = True
            else:
                states[new_state] = i
            i += 1

        return sum(sum(1 for _, yy in rocks if yy == y) * (h-y) for y in range(h))


def test_th_ch():
    """
    Run `python -m pytest ./day-14/part-2/th-ch.py` to test the submission.
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
#OO..#....
""".strip()
        )
        == 64
    )
