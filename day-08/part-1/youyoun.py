from tool.runners.python import SubmissionPy


class YouyounSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        sequence, map_ = s.split("\n\n")
        paths = {}
        for path in map_.splitlines():
            start, left_right = path.split(" = ")
            paths[start] = left_right.replace("(", "").replace(")", "").split(", ")

        start = "AAA"
        seq_pointer = 0
        n_steps = 0
        while start != "ZZZ":
            start = paths[start][int(sequence[seq_pointer] == "R")]
            seq_pointer = (seq_pointer + 1) % len(sequence)
            n_steps += 1
        return n_steps


def test_youyoun():
    """
    Run `python -m pytest ./day-08/part-1/youyoun.py` to test the submission.
    """
    assert (
        YouyounSubmission().run(
            """RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)""".strip()
        )
        == 2
    )

    assert (
        YouyounSubmission().run(
            """LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)""".strip()
        )
        == 6
    )
