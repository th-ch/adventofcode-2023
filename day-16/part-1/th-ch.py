from tool.runners.python import SubmissionPy

from collections import defaultdict, deque

UP, DOWN, LEFT, RIGHT = "up", "down", "left", "right"

def move_beam(beam, grid, w, h):
    x, y, direction = beam
    if direction == UP:
        next_x, next_y = x, y - 1
    elif direction == DOWN:
        next_x, next_y = x, y + 1
    elif direction == LEFT:
        next_x, next_y = x - 1, y
    elif direction == RIGHT:
        next_x, next_y = x + 1, y

    if not (0 <= next_y < h and 0 <= next_x < w):
        return []

    if grid[next_y][next_x] == "|":
        if direction == UP or direction == DOWN:
            return [(next_x, next_y, direction)]
        else:
            return [(next_x, next_y, UP), (next_x, next_y, DOWN)]
    elif grid[next_y][next_x] == "-":
        if direction == LEFT or direction == RIGHT:
            return [(next_x, next_y, direction)]
        else:
            return [(next_x, next_y, LEFT), (next_x, next_y, RIGHT)]
    elif grid[next_y][next_x] == "\\":
        if direction == UP:
            return [(next_x, next_y, LEFT)]
        elif direction == DOWN:
            return [(next_x, next_y, RIGHT)]
        elif direction == LEFT:
            return [(next_x, next_y, UP)]
        elif direction == RIGHT:
            return [(next_x, next_y, DOWN)]
    elif grid[next_y][next_x] == "/":
        if direction == UP:
            return [(next_x, next_y, RIGHT)]
        elif direction == DOWN:
            return [(next_x, next_y, LEFT)]
        elif direction == LEFT:
            return [(next_x, next_y, DOWN)]
        elif direction == RIGHT:
            return [(next_x, next_y, UP)]
    else:
        return [(next_x, next_y, direction)]

def count_energized(grid, w, h, start):
    beams = deque([start])
    seen = set()
    while beams:
        beam = beams.popleft()
        next_beams = move_beam(beam, grid, w, h)
        for next_beam in next_beams:
            if next_beam not in seen:
                seen.add(next_beam)
                beams.append(next_beam)

    energized = set((x, y) for x, y, _ in seen)
    return len(energized)

class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        grid = defaultdict(lambda: defaultdict(lambda: "."))
        for y, line in enumerate(s.splitlines()):
            for x, char in enumerate(line):
                if char == ".":
                    continue
                grid[y][x] = char
        w, h = x + 1 , y + 1

        return count_energized(grid, w, h, (-1, 0, RIGHT))


def test_th_ch():
    """
    Run `python -m pytest ./day-16/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            r"""
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
""".strip()
        )
        == 46
    )
