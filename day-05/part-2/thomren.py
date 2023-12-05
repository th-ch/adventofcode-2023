from itertools import chain
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str) -> int:
        """
        :param s: input in string format
        :return: solution flag
        """
        sections = s.split("\n\n")
        raw_ranges = [int(n) for n in sections[0].split(": ")[1].split(" ")]
        ranges = [
            (start, start + length)
            for start, length in zip(raw_ranges[::2], raw_ranges[1::2])
        ]
        for section in sections[1:]:
            lines = section.splitlines()
            mappings = [tuple(map(int, line.split())) for line in lines[1:]]
            mappings = sorted(
                [(source, dest, length) for (dest, source, length) in mappings]
            )
            mapped_ranges = []
            for start, end in ranges:
                for source_start, dest_start, map_length in mappings:
                    if start == end:
                        break
                    if start < source_start:
                        mapped_ranges.append((start, min(source_start, end)))
                        start = min(source_start, end)
                    if end < source_start:
                        mapped_ranges.append((start, end))
                        start = end
                        break

                    intersect_start, intersect_end = max(start, source_start), min(
                        end, source_start + map_length
                    )
                    if intersect_start < intersect_end:
                        mapped_ranges.append(
                            (
                                dest_start + intersect_start - source_start,
                                dest_start + intersect_end - source_start,
                            )
                        )
                        start = intersect_end

                if end > start:
                    mapped_ranges.append((start, end))

            ranges = mapped_ranges
        return min(start for (start, _) in ranges)


def test_thomren():
    """
    Run `python -m pytest ./day-05/part-2/thomren.py` to test the submission.
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
        == 46
    )
