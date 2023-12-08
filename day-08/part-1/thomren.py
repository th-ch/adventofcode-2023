from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        lines = s.splitlines()
        instructions = lines[0]
        graph = {}
        for line in lines[2:]:
            node, children = line.split(" = ")
            children = children[1:-1].split(", ")
            graph[node] = children

        steps = 0
        current = "AAA"
        while current != "ZZZ":
            if instructions[steps % len(instructions)] == "L":
                current = graph[current][0]
            else:
                current = graph[current][1]
            steps += 1
        return steps


def test_thomren():
    """
    Run `python -m pytest ./day-08/part-1/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
""".strip()
        )
        == 2
    )
    assert (
        ThomrenSubmission().run(
            """
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
""".strip()
        )
        == 6
    )
