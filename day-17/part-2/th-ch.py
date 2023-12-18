from tool.runners.python import SubmissionPy

from collections import defaultdict
from math import inf
from queue import PriorityQueue

UP, DOWN, LEFT, RIGHT = "U", "D", "L", "R"


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        grid = []
        for y, line in enumerate(s.splitlines()):
            grid.append([])
            for x,  c in enumerate(line):
                grid[y].append(int(c))
        w, h = x+1, y+1

        source = (0, 0, None, 0) # x, y, direction, nb_in_direction
        dist = defaultdict(lambda: inf)
        Q = PriorityQueue()
        dist[source] = 0
        for y in range(h):
            for x in range(w):
                if x == 0 and y == 0:
                    Q.put((0, source))
                else:
                    for direction in [UP, DOWN, LEFT, RIGHT]:
                        for nb_in_direction in range(1, 10):
                            Q.put((dist[(x, y, direction, nb_in_direction)], (x, y, direction, nb_in_direction)))


        while not Q.empty():
            _, (x, y, direction, nb_in_direction) = Q.get()

            neighbors = [(xx, yy, new_direction) for xx, yy, new_direction in [(x-1, y, LEFT), (x, y-1, UP), (x+1, y, RIGHT), (x, y+1, DOWN)] if 0 <= xx < w and 0 <= yy < h]
            if direction:
                reverse_direction = {UP: DOWN, DOWN: UP, LEFT: RIGHT, RIGHT: LEFT}[direction]
                neighbors = [(xx, yy, new_direction) for xx, yy, new_direction in neighbors if new_direction != reverse_direction]
            if 1 <= nb_in_direction < 4:
                # forced to go in that direction
                neighbors = [(xx, yy, new_direction) for xx, yy, new_direction in neighbors if new_direction == direction]
            elif nb_in_direction > 9:
                # forced to change direction
                neighbors = [(xx, yy, new_direction) for xx, yy, new_direction in neighbors if new_direction != direction]

            for xx, yy, new_direction in neighbors:
                if new_direction != direction or direction is None:
                    new_nb_in_direction = 1
                else:
                    new_nb_in_direction = nb_in_direction + 1

                alt = dist[(x, y, direction, nb_in_direction)] + grid[yy][xx]
                if alt < dist[(xx, yy, new_direction, new_nb_in_direction)]:
                    dist[(xx, yy, new_direction, new_nb_in_direction)] = alt
                    Q.put((alt, (xx, yy, new_direction, new_nb_in_direction)))

        d = inf
        for direction in [UP, DOWN, LEFT, RIGHT]:
            for nb_in_direction in range(1, 10):
                if dist[(w-1, h-1, direction, nb_in_direction)] < d:
                    d = dist[(w-1, h-1, direction, nb_in_direction)]
        return d


def test_th_ch():
    """
    Run `python -m pytest ./day-17/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
""".strip()
        )
        == 94
    )
