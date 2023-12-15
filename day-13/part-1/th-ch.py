from tool.runners.python import SubmissionPy


def find_reflections(pattern: str):
    reflections_x = set()
    reflections_y = set()
    rocks = set()
    for y, line in enumerate(pattern.splitlines()):
        for x, c in enumerate(line):
            if c == "#":
                rocks.add((x,y))
    w, h = x, y

    # vertical
    for reflection_x in range(0, w):
        is_reflection = True
        for x,y in rocks:
            reflected_x = 2 * reflection_x - x + 1
            if 0 <= reflected_x <= w and (reflected_x, y) not in rocks:
                is_reflection = False
                break
        if is_reflection:
            reflection_x += 1
            reflections_x.add(reflection_x)

    # horizontal
    for reflection_y in range(0, h):
        is_reflection = True
        for x,y in rocks:
            reflected_y = 2 * reflection_y - y + 1
            if 0 <= reflected_y <= h and (x, reflected_y) not in rocks:
                is_reflection = False
                break
        if is_reflection:
            reflection_y += 1
            reflections_y.add(reflection_y)

    return reflections_x, reflections_y



class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        patterns = s.split("\n\n")
        result = 0
        for pattern in patterns:
            reflections_x, reflections_y = find_reflections(pattern)
            if reflections_x:
                result += min(reflections_x)
            if reflections_y:
                result += min(reflections_y) * 100

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
        == 405
    )
