from math import lcm
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

        cycles = [
            find_z_cycle(graph, node, instructions)
            for node in graph
            if node.endswith("A")
        ]
        return lcm(*cycles)


def find_z_cycle(
    graph: dict[str, tuple[str, str]], start: str, instructions: str
) -> int:
    current = start
    steps = 0
    z_found = None
    z_found_steps = None
    while steps < 1000000:
        if current.endswith("Z"):
            if current == z_found:
                cycle_length = steps - z_found_steps
                if z_found_steps != cycle_length:
                    raise NotImplementedError(
                        "Cycle length is not equal to cycle start"
                    )
                return cycle_length
            elif z_found is not None:
                raise NotImplementedError("Cycle with multiple z nodes found")
            else:
                z_found = current
                z_found_steps = steps
        if instructions[steps % len(instructions)] == "L":
            current = graph[current][0]
        else:
            current = graph[current][1]
        steps += 1

    raise NotImplementedError("No cycle found")


def test_thomren():
    """
    Run `python -m pytest ./day-08/part-2/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
""".strip()
        )
        == 7
    )
