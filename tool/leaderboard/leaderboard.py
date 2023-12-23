from collections import defaultdict
from jinja2 import Environment, FileSystemLoader
import itertools

from tool.model import Result

templateLoader = FileSystemLoader(searchpath="./tool/leaderboard/")
templateEnv = Environment(loader=templateLoader)

template = templateEnv.get_template("leaderboard_template.html")


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
            results_by_part_by_day[day][part]
            for part in sorted(results_by_part_by_day[day])
        ]
        for day in sorted(results_by_part_by_day)
    ]


def generate_leaderboard(results: list[list[Result]]) -> None:
    rendered = template.render(res=group_by_day_and_part(results))
    with open("./leaderboard/index.html", "w+") as f:
        f.write(rendered)
