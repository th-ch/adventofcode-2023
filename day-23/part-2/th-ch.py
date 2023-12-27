from tool.runners.python import SubmissionPy

from collections import defaultdict, OrderedDict


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        lines = s.split("\n")
        start = None
        end = None
        paths = set()
        for y, line in enumerate(lines):
            for x, c in enumerate(line):
                if c in ".v^<>":
                    paths.add((x, y))
                if y == 0 and c == ".":
                    start = (x, y)
                elif y == len(lines)-1 and c == ".":
                    end = (x, y)

        neighbors = defaultdict(set)
        for x, y in paths:
            for xx, yy in [(x-1, y), (x+1, y), (x, y-1), (x, y+1)]:
                if (xx, yy) in paths:
                    neighbors[(x, y)].add((xx, yy))
                    neighbors[(xx, yy)].add((x, y))

        # Compress graph
        compressed_dist = defaultdict(lambda: defaultdict(lambda: 1))
        for x, y in paths:
            if len(neighbors[(x, y)]) == 2:
                n1 = neighbors[(x, y)].pop()
                n2 = neighbors[(x, y)].pop()
                neighbors[n1].remove((x, y))
                neighbors[n2].remove((x, y))
                neighbors[n1].add(n2)
                neighbors[n2].add(n1)
                del neighbors[(x, y)]
                compressed_dist[n1][n2] = compressed_dist[n1][(x, y)] + compressed_dist[(x, y)][n2]
                compressed_dist[n2][n1] = compressed_dist[n1][(x, y)] + compressed_dist[(x, y)][n2]

        # DFS for longest path
        cache = {}
        def dfs(x, y, seen):
            seen[(x, y)] = None
            if (x, y) == end:
                return 0, seen

            if (x, y, tuple(seen.keys())) in cache:
                return cache[(x, y, tuple(seen.keys()))]

            max_d = 0
            max_seen = OrderedDict()
            for xx, yy in neighbors[(x, y)]:
                if (xx, yy) in seen:
                    continue

                new_d, new_seen = dfs(xx, yy, seen.copy())
                new_d = compressed_dist[(x, y)][(xx,yy)]+new_d
                if new_d > max_d and end in new_seen:
                    max_d = new_d
                    max_seen = new_seen

            cache[(x, y, tuple(max_seen.keys()))] = max_d, max_seen
            return max_d, max_seen

        seen = OrderedDict()
        d, _ = dfs(start[0], start[1], seen)
        return d


def test_th_ch():
    """
    Run `python -m pytest ./day-23/part-2/th-ch.py` to test the submission.
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
        == 154
    )
