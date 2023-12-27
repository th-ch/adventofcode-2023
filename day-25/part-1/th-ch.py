from tool.runners.python import SubmissionPy

import networkx as nx
import matplotlib.pyplot as plt

def display_graph(G):
    pos = nx.spring_layout(G)
    nx.draw_networkx_labels(G, pos)
    nx.draw_networkx_edges(G, pos, edgelist=G.edges())
    plt.show()


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        G = nx.Graph()
        for line in s.splitlines():
            src, dst = line.split(": ")
            G.add_edges_from([(src, d) for d in dst.split()])

        cutset = nx.minimum_edge_cut(G)
        G.remove_edges_from(cutset)
        subgraphs = nx.connected_components(G)
        sizes = [len(subgraph) for subgraph in subgraphs]
        return sizes[0]*sizes[1]


def test_th_ch():
    """
    Run `python -m pytest ./day-25/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
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
