from tool.runners.python import SubmissionPy

from collections import defaultdict
from itertools import count
import numpy as np
from string import ascii_uppercase


class Brick():
    def __init__(self, x1 : int, y1 : int, z1 : int, x2 : int, y2 : int, z2 : int):
        self.x1 = min(x1, x2)
        self.y1 = min(y1, y2)
        self.z1 = min(z1, z2)
        self.x2 = max(x1, x2)
        self.y2 = max(y1, y2)
        self.z2 = max(z1, z2)

    def __str__(self) -> str:
        id = ascii_uppercase[self.id-1] if self.id < len(ascii_uppercase) else self.id
        return f"{id}: ({self.x1}, {self.y1}, {self.z1}) ~ ({self.x2}, {self.y2}, {self.z2})"


def fall(bricks: list, grid: np.ndarray):
    bricks_by_z = sorted(bricks, key=lambda brick: (brick.z1, brick.z2))
    for brick in bricks_by_z:
        while True:
            if brick.z1 <= 1:
                break
            if np.any(grid[brick.x1:brick.x2+1, brick.y1:brick.y2+1, brick.z1-1:brick.z1]):
                break
            grid[brick.x1:brick.x2+1, brick.y1:brick.y2+1, brick.z2:brick.z2+1] = 0
            brick.z1 -= 1
            brick.z2 -= 1
            grid[brick.x1:brick.x2+1, brick.y1:brick.y2+1, brick.z1:brick.z1+1] = brick.id
    return grid


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        bricks = []
        brick_ids = count(1)
        for line in s.split("\n"):
            start, end = line.split("~")
            x1, y1, z1 = map(int, start.split(","))
            x2, y2, z2 = map(int, end.split(","))
            brick = Brick(x1, y1, z1, x2, y2, z2)
            brick.id = next(brick_ids)
            bricks.append(brick)

        grid = np.zeros((max(brick.x2 for brick in bricks)+1, max(brick.y2 for brick in bricks)+1, max(brick.z2 for brick in bricks)+1), dtype=int)
        for brick in bricks:
            grid[brick.x1:brick.x2+1, brick.y1:brick.y2+1, brick.z1:brick.z2+1] = brick.id
        grid = fall(bricks, grid)

        removable_bricks = set()
        supported = defaultdict(set)
        supporting = defaultdict(set)
        for brick in bricks:
            on_top = set(np.unique(grid[brick.x1:brick.x2+1, brick.y1:brick.y2+1, brick.z2+1:brick.z2+2])) - {0}
            supporting[brick.id] = on_top
            for other_id in on_top:
                supported[other_id].add(brick.id)

        for brick in bricks:
            if all(len(supported[supported_brick_id])>1 for supported_brick_id in supporting[brick.id]):
                removable_bricks.add(brick.id)
        return len(removable_bricks)


def test_th_ch():
    """
    Run `python -m pytest ./day-22/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
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
