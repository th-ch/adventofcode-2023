from dataclasses import dataclass, field
from itertools import count

import numpy as np
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        # parse bricks
        bricks = []
        for line in s.splitlines():
            p1, p2 = line.split("~")
            x1, y1, z1 = map(int, p1.split(","))
            x2, y2, z2 = map(int, p2.split(","))
            x1, x2 = sorted([x1, x2])
            y1, y2 = sorted([y1, y2])
            z1, z2 = sorted([z1, z2])
            bricks.append(Brick(x1, y1, z1, x2 + 1, y2 + 1, z2 + 1))

        # create a grid with bricks
        xmax = max(brick.x2 for brick in bricks)
        ymax = max(brick.y2 for brick in bricks)
        zmax = max(brick.z2 for brick in bricks)
        grid = np.zeros((xmax, ymax, zmax), dtype=int)
        for brick in bricks:
            grid[
                brick.x1 : brick.x2,
                brick.y1 : brick.y2,
                brick.z1 : brick.z2,
            ] = brick.id
        grid[:, :, 0] = 1000000  # just needs to be >0

        # simulate bricks falling
        moved = True
        while moved:
            moved = False
            for brick in sorted(bricks, key=lambda b: b.z1):
                if (
                    np.sum(grid[brick.x1 : brick.x2, brick.y1 : brick.y2, brick.z1 - 1])
                    == 0
                ):
                    brick.z1 -= 1
                    brick.z2 -= 1
                    grid[brick.x1 : brick.x2, brick.y1 : brick.y2, brick.z2] = 0
                    grid[brick.x1 : brick.x2, brick.y1 : brick.y2, brick.z1] = brick.id
                    moved = True

        # find unnecessary bricks
        unnecessary_bricks = set(b.id for b in bricks)
        for brick in bricks:
            supported_by = set(
                np.unique(grid[brick.x1 : brick.x2, brick.y1 : brick.y2, brick.z1 - 1])
            ) - {0}
            if len(supported_by) == 1:
                unnecessary_bricks -= {next(iter(supported_by))}

        return len(unnecessary_bricks)


@dataclass
class Brick:
    x1: int
    y1: int
    z1: int
    x2: int
    y2: int
    z2: int
    id: int = field(default_factory=lambda counter=count(): next(counter) + 1)


def test_thomren():
    """
    Run `python -m pytest ./day-22/part-1/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
""".strip()
        )
        == 5
    )
