from tool.runners.python import SubmissionPy

from collections import defaultdict
from math import inf
from queue import PriorityQueue

from matplotlib import path
import numpy as np


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        grid = [[c for c in line] for line in s.splitlines()]

        # Find the starting point
        y_s = None
        for y, line in enumerate(grid):
            for x_s, c in enumerate(line):
                if c == "S":
                    y_s = y
                    break
            if y_s is not None:
                break

        # Compute the distance matrix
        scores = [[inf for _ in line] for line in grid]
        nexts = defaultdict(set)
        scores[y_s][x_s] = 0

        points = PriorityQueue()
        for y in range(len(grid)):
            for x in range(len(grid[y])):
                if grid[y][x] != ".":
                    points.put((scores[y][x], (x, y)))

        while not points.empty():
            _, (x, y) = points.get()
            next_points = []
            # left
            if x>0 and grid[y][x] in ["S", "-", "J", "7"] and grid[y][x-1] in ["-", "L", "F"]:
                next_points.append((x-1, y))
            # right
            if x<len(grid[y])-1 and grid[y][x] in ["S", "-", "L", "F"] and grid[y][x+1] in ["-", "J", "7"]:
                next_points.append((x+1, y))
            # down
            if y<len(grid)-1 and grid[y][x] in ["S", "|", "7", "F"] and grid[y+1][x] in ["|", "L", "J"]:
                next_points.append((x, y+1))
            # up
            if y>0 and grid[y][x] in ["S", "|", "L", "J"] and grid[y-1][x] in ["|", "7", "F"]:
                next_points.append((x, y-1))

            for xx, yy in next_points:
                alt = scores[y][x] + 1
                if alt < scores[yy][xx]:
                    scores[yy][xx] = alt
                    points.put((alt, (xx, yy)))
                    nexts[(x, y)].add((xx, yy))

        # Compute the 2 paths from S to the farthest point to get the loop
        farthest = max(max(d if d != inf else -inf for d in line) for line in scores)
        path1 = [(x_s, y_s)]
        path2 = [(x_s, y_s)]
        for _ in range(farthest):
            for next1 in nexts[path1[-1]]:
                if next1 not in path1:
                    path1.append(next1)
                    break
            for next2 in nexts[path2[-1]]:
                if next2 != next1 and next2 not in path2:
                    path2.append(next2)
                    break
        path2.reverse()
        loop = path1 + path2[:-1]

        # Compute the number of points inside the loop with matplotlib
        mask = np.array([[x, y] for y in range(len(grid)) for x in range(len(grid[y])) if (x, y) not in loop])
        p = path.Path(loop)
        points_inside = p.contains_points(mask)
        return points_inside.sum()


def test_th_ch():
    """
    Run `python -m pytest ./day-10/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
""".strip()
        )
        == 1
    )

    assert (
        ThChSubmission().run(
            """
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
""".strip()
        )
        == 4
    )

    assert (
        ThChSubmission().run(
            """
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
""".strip()
        )
        == 8
    )

    assert (
        ThChSubmission().run(
            """
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
""".strip()
        )
        == 10
    )
