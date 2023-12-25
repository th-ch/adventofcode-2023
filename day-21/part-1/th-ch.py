from tool.runners.python import SubmissionPy

from queue import Queue

def compute_distances(rocks, plots, x_start, y_start, nb_steps):
    q = Queue()
    q.put((x_start, y_start, 0))
    visited = {}
    while not q.empty():
        x, y, steps = q.get()
        if (x, y) in visited:
            continue
        visited[(x, y)] = steps
        if steps == nb_steps:
            continue
        for dx, dy in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
            x2, y2 = x + dx, y + dy
            if (x2, y2) in rocks:
                continue
            if (x2, y2) in plots:
                q.put((x2, y2, steps + 1))
    return visited


class ThChSubmission(SubmissionPy):
    def run(self, s: str, nb_steps=64):
        """
        :param s: input in string format
        :return: solution flag
        """
        plots = set()
        rocks = set()
        x_start, y_start = None, None
        for y, line in enumerate(s.splitlines()):
            for x, c in enumerate(line):
                if c == "S":
                    x_start, y_start = x, y
                elif c == "#":
                    rocks.add((x, y))
                elif c == ".":
                    plots.add((x, y))

        visited = compute_distances(rocks, plots, x_start, y_start, nb_steps)
        return sum(1 for v in visited.values() if v <= nb_steps and v % 2 == 0)


def test_th_ch():
    """
    Run `python -m pytest ./day-21/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
""".strip(), nb_steps=6
        )
        == 16
    )
