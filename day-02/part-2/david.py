from tool.runners.python import SubmissionPy

from dataclasses import dataclass
from typing import Self

@dataclass
class Draw:
    red: int = 0
    blue: int = 0
    green: int = 0

    def power(self) -> int:
        return self.red * self.blue * self.green
    
    def merge(self, other: Self):
        self.red = max(self.red, other.red)
        self.blue = max(self.blue, other.blue)
        self.green = max(self.green, other.green)

    @staticmethod
    def from_str(raw_draws: list[str]) -> Self:
        draw = Draw()
        for raw_draw in raw_draws:
            count, color = raw_draw.split(" ")
            count = int(count)
            if color == 'red':
                draw.red = count
            elif color == 'green':
                draw.green = count
            elif color == 'blue':
                draw.blue = count
            else:
                raise Exception(f"invalid color: {raw_draw}")
        return draw 

class DavidSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        lines = s.split("\n")
        result = 0
        for line in lines:
            _, content = line.split(": ")
            root_draw = Draw()
            for raw_draws in content.split("; "):
                draw = Draw.from_str(raw_draws.split(", "))
                root_draw.merge(draw)
            power = root_draw.power()
            result += power

        return result
