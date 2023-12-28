from collections import defaultdict
import dataclasses
from typing import NamedTuple
from jinja2 import Environment, FileSystemLoader
import itertools

from tool.model import Result

templateLoader = FileSystemLoader(searchpath="./tool/leaderboard/")
templateEnv = Environment(loader=templateLoader)

template = templateEnv.get_template("leaderboard_template.html")


class AggregatedAuthorResult(NamedTuple):
    author: str
    languages: list[str]
    stars: int
    total_execution_time: float


class LeaderboardData(NamedTuple):
    results_by_day_and_part: list[list[list[Result]]]
    aggregated_author_results: list[AggregatedAuthorResult]


def group_by_day_and_part(
    results: list[list[Result]],
) -> list[list[list[Result]]]:
    results_by_part_by_day: defaultdict[
        int, defaultdict[int, list[Result]]
    ] = defaultdict(lambda: defaultdict(list))
    for result in itertools.chain.from_iterable(results):
        results_by_part_by_day[result.problem.day][result.problem.part].append(result)
    return [
        [
            sorted(results_by_part_by_day[day][part], key=lambda r: r.duration)
            for part in sorted(results_by_part_by_day[day])
        ]
        for day in sorted(results_by_part_by_day)
    ]


@dataclasses.dataclass
class CumulativeAuthorResults:
    languages: set[str] = dataclasses.field(default_factory=set)
    stars: int = 0
    total_execution_time: float = 0.0


def aggregate_author_results(
    results_by_day_and_part: list[list[list[Result]]],
) -> list[AggregatedAuthorResult]:
    results_by_author: defaultdict[str, CumulativeAuthorResults] = defaultdict(
        CumulativeAuthorResults
    )
    for results_by_part in results_by_day_and_part:
        for results in results_by_part:
            # Ignore day 25 part 2, which is given if all 49 other problems were
            # solved
            if not results or (
                results[0].problem.day == 25 and results[0].problem.part == 2
            ):
                continue
            best_result_by_author: dict[str, Result] = {}
            for result in results:
                author = result.submission.author
                if (
                    author not in best_result_by_author
                    or result.duration < best_result_by_author[author].duration
                ):
                    best_result_by_author[author] = result
            for author, result in best_result_by_author.items():
                results_by_author[author].languages.add(result.submission.language)
                results_by_author[author].stars += 1
                results_by_author[author].total_execution_time += result.duration
    return sorted(
        (
            AggregatedAuthorResult(
                author,
                sorted(c.languages),
                50 if c.stars == 49 else c.stars,
                c.total_execution_time,
            )
            for author, c in results_by_author.items()
        ),
        key=lambda a: (-a.stars, a.total_execution_time),
    )


def build_leaderboard_data(results: list[list[Result]]) -> LeaderboardData:
    results_by_day_and_part = group_by_day_and_part(results)
    res = LeaderboardData(
        results_by_day_and_part, aggregate_author_results(results_by_day_and_part)
    )
    return res


def generate_leaderboard(results: list[list[Result]]) -> None:
    rendered = template.render(res=build_leaderboard_data(results))
    with open("./leaderboard/index.html", "w+") as f:
        f.write(rendered)
