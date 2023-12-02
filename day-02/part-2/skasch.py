from tool.runners.python import SubmissionPy
import dataclasses


@dataclasses.dataclass
class MinCubes:
    R: int = 0
    G: int = 0
    B: int = 0

    def Update(self, count: int, color: str) -> None:
        if color == "red":
            self.R = max(self.R, count)
        elif color == "blue":
            self.B = max(self.B, count)
        elif color == "green":
            self.G = max(self.G, count)

    def Power(self) -> int:
        return self.R * self.G * self.B


def GetPower(line: str) -> int:
    min_cubes = MinCubes()
    for draw_str in line[line.find(": ") + 2 :].split("; "):
        for cubes in draw_str.split(", "):
            count, color = cubes.split(" ")
            min_cubes.Update(int(count), color)
    return min_cubes.Power()


class SkaschSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        result = 0
        for line in s.splitlines():
            result += GetPower(line)
        return str(result)


def test_skasch():
    """
    Run `python -m pytest ./day-02/part-2/skasch.py` to test the submission.
    """
    assert (
        SkaschSubmission().run(
            """
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
""".strip()
        )
        == "2286"
    )
