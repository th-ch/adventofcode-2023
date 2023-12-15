from tool.runners.python import SubmissionPy
import math

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

        starts = [s for s in paths if "A" in s]
        arrives_at = [0 for _ in starts]
        seq_pointer = 0

        for i in range(len(starts)):
            n_steps = 0
            start = starts[i]
            while start[-1] != "Z":
                start = paths[start][int(sequence[seq_pointer] == "R")]
                seq_pointer = (seq_pointer + 1) % len(sequence)
                n_steps += 1
            arrives_at[i] = n_steps
        return math.lcm(*arrives_at)

def test_youyoun():
    """
    Run `python -m pytest ./day-08/part-2/youyoun.py` to test the submission.
    """
    assert (
        YouyounSubmission().run(
            """LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)""".strip()
        )
        == 6
    )
