from tool.runners.python import SubmissionPy


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

        nb_steps = 0
        current = "AAA"
        i = 0
        while current != "ZZZ":
            current = graph[current][path[i]]
            i = (i + 1) % len(path)
            nb_steps += 1
        return nb_steps



def test_th_ch():
    """
    Run `python -m pytest ./day-08/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
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
