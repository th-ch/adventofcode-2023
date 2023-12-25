from tool.runners.python import SubmissionPy
from networkx import Graph, minimum_edge_cut, connected_components


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        g = Graph()
        for line in s.splitlines():
            vertex, neighbors = line.split(": ")
            for neighbor in neighbors.split(" "):
                g.add_edge(vertex, neighbor)
        cutset = minimum_edge_cut(g)
        assert len(cutset) == 3
        g.remove_edges_from(cutset)
        components = list(connected_components(g))
        assert len(components) == 2
        return len(components[0]) * len(components[1])


def test_thomren():
    """
    Run `python -m pytest ./day-25/part-1/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
""".strip()
        )
        == 54
    )
