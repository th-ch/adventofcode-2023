from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        maps = s.split("\n\n")
        seeds = list(map(int, maps[0].replace("seeds: ", "").split(" ")))
        location_by_seed = {seed: seed for seed in seeds}
        for section in maps[1:]:
            mappings = section.split("\n")[1:]
            processed_seeds= set()
            for mapping in mappings:
                dest_start, source_start, length = map(int, mapping.split())
                for seed in location_by_seed:
                    if seed not in processed_seeds and source_start <= location_by_seed[seed] < source_start + length:
                        location_by_seed[seed] = dest_start + location_by_seed[seed] - source_start
                        processed_seeds.add(seed)

        return min(location_by_seed.values())


def test_th_ch():
    """
    Run `python -m pytest ./day-05/part-1/th-ch.py` to test the submission.
    """
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
        == 35
    )
