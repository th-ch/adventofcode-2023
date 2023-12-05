from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str) -> int:
        """
        :param s: input in string format
        :return: solution flag
        """
        sections = s.split("\n\n")
        ids = [int(n) for n in sections[0].split(": ")[1].split(" ")]
        for section in sections[1:]:
            lines = section.splitlines()
            mappings = [tuple(map(int, line.split())) for line in lines[1:]]
            for i, n in enumerate(ids):
                for mapping in mappings:
                    if mapping[1] <= n < mapping[1] + mapping[2]:
                        ids[i] = mapping[0] + n - mapping[1]
                        break
        return min(ids)


def test_thomren():
    """
    Run `python -m pytest ./day-05/part-1/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
""".strip()
        )
        == 35
    )
