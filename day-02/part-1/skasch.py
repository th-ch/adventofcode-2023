from tool.runners.python import SubmissionPy
import dataclasses


@dataclasses.dataclass
class Draw:
    R: int = 0
    G: int = 0
    B: int = 0

    def Add(self, count: int, color: str) -> None:
        if color == "red":
            self.R += count
        elif color == "blue":
            self.B += count
        elif color == "green":
            self.G += count

    def __le__(self, other: "Draw") -> bool:
        return self.R <= other.R and self.G <= other.G and self.B <= other.B


BAG = Draw(12, 13, 14)


def IsValid(line: str) -> bool:
    for draw_str in line[line.find(": ") + 2 :].split("; "):
        draw = Draw()
        for cubes in draw_str.split(", "):
            count, color = cubes.split(" ")
            draw.Add(int(count), color)
        if not draw <= BAG:
            return False
    return True


class SkaschSubmission(SubmissionPy):
    def run(self, s: str) -> str:
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        result = 0
        for id, line in enumerate(s.splitlines()):
            if IsValid(line.strip()):
                result += id + 1
        return str(result)


def test_skasch():
    """
    Run `python -m pytest ./day-02/part-1/skasch.py` to test the submission.
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
        == "8"
    )
