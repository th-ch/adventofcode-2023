from collections import defaultdict, deque
from functools import cache
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        grid = [[c for c in line] for line in s.split("\n")]
        start = (0, next(i for i, c in enumerate(grid[0]) if c == "."))
        end = (len(grid) - 1, next(i for i, c in enumerate(grid[-1]) if c == "."))

        # build condensed graph
        stack = [start]
        visited = set()
        graph = defaultdict(set)
        while stack:
            pos = stack.pop()
            if pos == end or pos in visited:
                continue
            visited.add(pos)

            for neighbor in get_neighbors(grid, pos):
                (nx, ny), nd = find_next_intersection(
                    grid, neighbor, (neighbor[0] - pos[0], neighbor[1] - pos[1])
                )
                graph[pos].add(((nx, ny), nd))
                stack.append((nx, ny))

        # find longest path
        stack = [(start, 0)]
        visited = set()
        max_dist = 0
        while stack:
            pos, dist = stack.pop()
            if pos == end:
                max_dist = max(max_dist, dist)
                continue
            elif dist == -1:
                visited.remove(pos)
                continue
            elif pos in visited:
                continue
            visited.add(pos)

            # will backtrack once all the paths from pos have been explored
            stack.append((pos, -1))

            for neighbor, d in graph[pos]:
                if neighbor not in visited:
                    stack.append((neighbor, dist + d))

        return max_dist


def find_next_intersection(grid, pos, direction):
    dist = 0
    x, y = pos
    dx, dy = direction
    neighbors = set(get_neighbors(grid, pos)) - {(x - dx, y - dy)}
    while len(neighbors) == 1:
        neighbor = next(n for n in neighbors if n != (x - dx, y - dy))
        dx, dy = (neighbor[0] - x, neighbor[1] - y)
        x, y = neighbor
        dist += 1
        neighbors = set(get_neighbors(grid, (x, y))) - {(x - dx, y - dy)}
    return (x, y), dist + 1


def get_neighbors(grid: list[list[str]], pos: tuple[int, int]) -> list[tuple[int, int]]:
    for dx, dy in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
        x, y = (pos[0] + dx, pos[1] + dy)
        if not 0 <= x < len(grid) or not 0 <= y < len(grid[0]) or grid[x][y] == "#":
            continue
        yield (x, y)


def test_thomren():
    """
    Run `python -m pytest ./day-23/part-2/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
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
