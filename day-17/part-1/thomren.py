from functools import cache
import heapq
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        heat_losses = [[int(c) for c in line] for line in s.splitlines()]
        end = (len(heat_losses) - 1, len(heat_losses[0]) - 1)

        queue = [(0, (0, 0), (0, 0), 2)]
        visited = set()
        while len(queue) > 0:
            d, (x, y), (dx, dy), r = heapq.heappop(queue)
            if (x, y) == end:
                return d
            if (x, y, dx, dy, r) in visited:
                continue
            visited.add((x, y, dx, dy, r))
            for ndx, ndy in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
                nx, ny = x + ndx, y + ndy
                if (
                    not (0 <= nx < len(heat_losses))
                    or not (0 <= ny < len(heat_losses[0]))
                    or (ndx, ndy) == (-dx, -dy)
                    or ((ndx, ndy) == (dx, dy) and r == 0)
                ):
                    continue
                heapq.heappush(
                    queue,
                    (
                        d + heat_losses[nx][ny],
                        (nx, ny),
                        (ndx, ndy),
                        r - 1 if (ndx, ndy) == (dx, dy) else 2,
                    ),
                )
        raise ValueError("Could not find solution")


def test_thomren():
    """
    Run `python -m pytest ./day-17/part-1/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
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
        == 102
    )
