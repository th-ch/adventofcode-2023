from tool.runners.python import SubmissionPy

from collections import defaultdict
from math import inf
from queue import PriorityQueue


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        lines = s.split("\n")
        grid = []
        start = None
        end = None
        paths = set()
        for y, line in enumerate(lines):
            grid.append([])
            for x, c in enumerate(line):
                grid[y].append(c if c != "#" else None)
                if c != "#":
                    paths.add((x, y))
                if y == 0 and c == ".":
                    start = (x, y)
                elif y == len(lines)-1 and c == ".":
                    end = (x, y)

        dist = defaultdict(lambda: inf)
        prev = defaultdict(lambda: None)
        dist[start] = 0
        Q = PriorityQueue()
        for x, y in paths:
            Q.put((dist[(x, y)], (x, y)))

        while not Q.empty():
            _, (x, y) = Q.get()
            for xx, yy, slope in [(x-1, y, "<"), (x+1, y, ">"), (x, y-1, "^"), (x, y+1, "v")]:
                if (xx, yy) not in paths:
                    continue
                if grid[yy][xx] != "." and grid[yy][xx] != slope:
                    continue
                if prev[(x, y)] == (xx, yy):
                    continue

                alt = dist[(x, y)] - 1
                if alt < dist[(xx, yy)]:
                    dist[(xx, yy)] = alt
                    prev[(xx, yy)] = (x, y)
                    Q.put((alt, (xx, yy)))

        return -dist[end]


def test_th_ch():
    """
    Run `python -m pytest ./day-23/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
""".strip()
        )
        == 94
    )
