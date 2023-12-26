from tool.runners.python import SubmissionPy

from collections import defaultdict
from importlib import import_module
from itertools import count
import numpy as np
from queue import deque

part1 = import_module("day-22.part-1.th-ch")
Brick = part1.Brick
fall = part1.fall

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

        supported = defaultdict(set)
        supporting = defaultdict(set)
        for brick in bricks:
            on_top = set(np.unique(grid[brick.x1:brick.x2+1, brick.y1:brick.y2+1, brick.z2+1:brick.z2+2])) - {0}
            supporting[brick.id] = on_top
            for other_id in on_top:
                supported[other_id].add(brick.id)

        result = 0
        for brick in bricks:
            q = deque([brick.id])
            fallen = set()
            while q:
                brick_id = q.popleft()
                fallen.add(brick_id)
                supported_brick_ids = supporting[brick_id]
                for supported_brick_id in supported_brick_ids:
                    if len(supported[supported_brick_id] - fallen) == 0:
                        q.append(supported_brick_id)
            result += len(fallen) - 1
        return result


def test_th_ch():
    """
    Run `python -m pytest ./day-22/part-2/th-ch.py` to test the submission.
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
        == 7
    )
