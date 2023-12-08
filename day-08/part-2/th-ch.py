from tool.runners.python import SubmissionPy

from math import lcm


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        graph = {}
        path, links = s.split("\n\n")
        for link in links.splitlines():
            origin, left_right = link.split(" = ")
            graph[origin] = {}
            left, right = left_right[1:-1].split(", ")
            graph[origin]["L"] = left
            graph[origin]["R"] = right

        starts = [n for n in graph if n[-1] == "A"]
        finishes = {}
        for start in starts:
            nb_steps = 0
            current = start
            i = 0
            while True:
                current = graph[current][path[i]]
                i = (i + 1) % len(path)
                nb_steps += 1
                if current[-1] == "Z":
                    # end node
                    finishes[start] = nb_steps
                    break

        return lcm(*[nb_steps for nb_steps in finishes.values()])


def test_th_ch():
    """
    Run `python -m pytest ./day-08/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
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
        == 6
    )
