from tool.runners.python import SubmissionPy

from importlib import import_module


part1 = import_module("day-12.part-1.th-ch")


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        result = 0
        for line in s.splitlines():
            unknown, truth = line.split()
            truth = tuple(int(t) for t in truth.split(","))

            unknown = "?".join([unknown] * 5)
            truth *= 5

            nb = part1.compute_nb(unknown, truth)
            result += nb
        return result


def test_th_ch():
    """
    Run `python -m pytest ./day-12/part-2/th-ch.py` to test the submission.
    """
    assert part1.compute_nb("?###??????????###??????????###??????????###??????????###????????", (3,2,1,3,2,1,3,2,1,3,2,1,3,2,1)) == 506250
    assert (
        ThChSubmission().run(
            """
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
""".strip()
        )
        == 525152
    )
