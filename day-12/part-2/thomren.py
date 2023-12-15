import functools

from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        res = 0
        for line in s.splitlines():
            springs, contiguous = line.split()
            springs = "?".join(springs for _ in range(5))
            contiguous = 5 * tuple(int(x) for x in contiguous.split(","))
            c = combinations(springs, contiguous)
            res += c
        return res


@functools.cache
def combinations(springs, contiguous):
    if len(springs) == 0:
        return int(len(contiguous) == 0)
    elif len(contiguous) == 0:
        return int(all(c in ".?" for c in springs))

    res = 0
    if springs[0] in ".?":
        res += combinations(springs[1:], contiguous)
    if (
        all(c in "#?" for c in springs[: contiguous[0]])
        and len(springs) >= contiguous[0]
        and (len(springs) == contiguous[0] or springs[contiguous[0]] in ".?")
    ):
        res += combinations(springs[contiguous[0] + 1 :], contiguous[1:])
    return res


def test_thomren():
    """
    Run `python -m pytest ./day-12/part-2/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
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
