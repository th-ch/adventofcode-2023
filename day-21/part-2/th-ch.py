from tool.runners.python import SubmissionPy

from queue import Queue

def compute_distances(rocks, plots, x_start, y_start, nb_steps, w, h):
    nb_grids = nb_steps // w
    all_plots = plots.copy()
    all_rocks = rocks.copy()
    for grid_y in range(-nb_grids, nb_grids + 1):
        for grid_x in range(-nb_grids, nb_grids + 1):
            for x, y in plots:
                x2, y2 = x + w * grid_x, y + h * grid_y
                all_plots.add((x2, y2))
            for x, y in rocks:
                x2, y2 = x + w * grid_x, y + h * grid_y
                all_rocks.add((x2, y2))
            # Add a plot instead of S
            all_plots.add((x_start + w * grid_x, y_start + h * grid_y))

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
            x2, y2 = x + dx , y + dy
            if (x2, y2) in all_rocks:
                continue
            if (x2, y2) in all_plots:
                q.put((x2, y2, steps + 1))
    return visited


class ThChSubmission(SubmissionPy):
    def run(self, s: str, nb_steps=26501365):
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
        w, h = x + 1, y + 1

        # 26501365 = 65 + 202300 * 131 (with 131 being the input size)
        # second degree diff is always the same
        # a b c
        #  d e
        #   f
        # c = b + e = b + (b-a) + f = 2 * b - a + f with f always the same

        first_values = []
        for x in range(3):
            new_d = 65 + x * w
            visited = compute_distances(rocks, plots, x_start, y_start, new_d, w, h)
            first_values.append(sum(1 for v in visited.values() if v <= new_d and v % 2 == new_d % 2))

        a, b, c = first_values[0], first_values[1], first_values[2]
        d, e = b-a, c-b
        f = e-d
        for _ in range(1, 202300):
            c = 2 * b - a + f
            a, b = b, c
        return c
