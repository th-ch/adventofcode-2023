from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        maps = s.split("\n\n")
        seeds = list(map(int, maps[0].replace("seeds: ", "").split(" ")))
        ranges = [
            (seeds[i], seeds[i] + seeds[i+1]) for i in range(0, len(seeds), 2)
        ]
        updated_ranges = {}

        for section in maps[1:]:
            mappings = section.split("\n")[1:]
            segments = {}
            for mapping in mappings:
                dest_start, source_start, length = map(int, mapping.split())
                segments[(source_start, source_start + length)] = dest_start

            updated_ranges = {}
            for beg, end in ranges:
                new_segments = split_segment((beg, end), sorted(segments.keys()))
                for new_beg, new_end in new_segments:
                    try:
                        value = next(segments[(a, b)] + new_beg - a for a,b in segments if a<=new_beg and new_end<=b)
                    except StopIteration:
                        value = new_beg
                    updated_ranges[(new_beg, new_end)] = value

            ranges = [(v, v+end-beg) for (beg, end), v in updated_ranges.items()]

        return min(updated_ranges.values())

def split_segment(segment_to_split, ordered_segments):
    start, end = segment_to_split
    if end<=start:
        return []
    for s, e in ordered_segments:
        if end<=s:
            continue
        if start >= e:
            continue
        split = []
        current_start = start
        if current_start < s:
            split.append((current_start, s))
            current_start = s
        if end > e:
            split.append((current_start, e))
        if split:
            return split + split_segment((split[-1][1], end), ordered_segments)
    return [(start, end)]


def test_th_ch():
    """
    Run `python -m pytest ./day-05/part-2/th-ch.py` to test the submission.
    """
    assert (
        split_segment((79, 92), [(60, 70), (80, 90)]) == [(79, 80), (80, 90), (90, 92)]
    )

    assert (
        ThChSubmission().run(
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
