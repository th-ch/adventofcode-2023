from collections import defaultdict, deque
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

            for neighbor in get_neighbors(grid, pos):
                if neighbor not in visited:
                    stack.append((neighbor, dist + 1))

        return max_dist


def get_neighbors(grid: list[list[str]], pos: tuple[int, int]) -> list[tuple[int, int]]:
    for dx, dy in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
        x, y = (pos[0] + dx, pos[1] + dy)
        if (
            not 0 <= x < len(grid)
            or not 0 <= y < len(grid[0])
            or grid[x][y] == "#"
            or (grid[x][y] == ">" and (dx, dy) != (0, 1))
            or (grid[x][y] == "<" and (dx, dy) != (0, -1))
            or (grid[x][y] == "^" and (dx, dy) != (-1, 0))
            or (grid[x][y] == "v" and (dx, dy) != (1, 0))
        ):
            continue
        yield (x, y)


def test_thomren():
    """
    Run `python -m pytest ./day-23/part-1/thomren.py` to test the submission.
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
        == 94
    )
