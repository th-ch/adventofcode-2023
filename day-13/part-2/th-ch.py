from tool.runners.python import SubmissionPy

from importlib import import_module

part1 = import_module("day-13.part-1.th-ch")


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        patterns = s.split("\n\n")
        result = 0
        for pattern in patterns:
            original_reflections_x, original_reflections_y = part1.find_reflections(pattern)
            for i, c in enumerate(pattern):
                new_c = ""
                if c == "#":
                    new_c = "."
                elif c == ".":
                    new_c = "#"

                if new_c:
                    smudged = pattern[:i] + new_c + pattern[i + 1:]
                    reflections_x, reflections_y = part1.find_reflections(smudged)
                    if (reflections_x or reflections_y) and (original_reflections_x != reflections_x or original_reflections_y != reflections_y):
                        new_reflections_x = reflections_x - original_reflections_x
                        new_reflections_y = reflections_y - original_reflections_y
                        if new_reflections_x:
                            result += min(new_reflections_x)
                        if new_reflections_y:
                            result += min(new_reflections_y) * 100
                        break

        return result


def test_th_ch():
    """
    Run `python -m pytest ./day-13/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
""".strip()
        )
        == 400
    )
