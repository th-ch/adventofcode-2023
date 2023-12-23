#!/usr/bin/env python3

import sys
import time
from collections import defaultdict
from typing import Mapping

from tabulate import tabulate

import tool.discovery as discovery
from tool.config import CONFIG
from tool.distribution import get_time_distribution
from tool.model import Input, Problem, Result, Submission
from tool.runners.wrapper import SubmissionWrapper
from tool.utils import BColor
from tool.leaderboard.leaderboard import generate_leaderboard


class DifferentAnswersException(Exception):
    pass


class UnexpectedDebugLinesException(Exception):
    pass


def run(
    days: list[int] | None,
    parts: list[int] | None,
    authors: list[str] | None,
    ignored_authors: list[str] | None,
    languages: list[str] | None,
    force: bool,
    no_debug: bool,
    all_days_parts: bool,
    restricted: bool,
    expand: bool,
    print_time_dist: bool,
) -> None:
    problems = discovery.get_problems(days, parts, all_days_parts)
    printed_day_header: set[int] = set()
    errors: list[str] = []
    all_results: list[list[Result]] = []

    for problem in problems:
        if problem.day not in printed_day_header:
            printed_day_header.add(problem.day)
            print_day_header(problem)
        print_part_header(problem)

        submissions = discovery.get_submissions(
            problem, authors, ignored_authors, languages, force
        )
        inputs = discovery.get_inputs(problem)

        results_by_author: defaultdict[str, list[Result]] = defaultdict(list)
        results_by_input: defaultdict[str, list[Result]] = defaultdict(list)

        for input in inputs:
            previous = None
            for submission in submissions:
                # The split allows having author.lang and author.x.lang files, on the same input
                if restricted and input.author != submission.author.split(".")[0]:
                    continue
                try:
                    result = run_submission(
                        problem, submission, input, previous, no_debug
                    )
                    results_by_author[submission.author].append(result)
                    results_by_input[input.author].append(result)
                    previous = result
                except (DifferentAnswersException, UnexpectedDebugLinesException) as e:
                    errors.append(
                        f"{BColor.RED}ERROR: {e}{BColor.ENDC}".format(
                            BColor.RED, e, BColor.ENDC
                        )
                    )

        for submission in submissions:
            if submission.runnable is not None:
                submission.runnable.cleanup()
        parser = problem.parser()
        if parser is not None:
            parser.cleanup()

        if restricted:
            print_restrict_results(results_by_author)
        elif expand:
            print_expanded_results(results_by_input)
        else:
            results = get_aggregated_results(
                problem, results_by_author, print_time_dist
            )
            print_aggregated_header()
            print_results(results, print_time_dist)
            all_results.append(results)

    for err in errors:
        print(err, file=sys.stderr)
    if errors:
        exit(1)

    if len(all_results) > 0 and all_days_parts:
        generate_leaderboard(results=all_results)


def run_submission(
    problem: Problem,
    submission: Submission,
    input: Input,
    previous: Result | None,
    no_debug: bool,
):
    start = time.perf_counter()
    assert submission.runnable is not None
    output = submission.runnable.run(input.content)
    end = time.perf_counter()
    msecs = (end - start) * 1000

    # TODO: SubmissionPy and SubmissionWrapper are fairly asymmetrical and need to be unified
    if isinstance(submission.runnable, SubmissionWrapper):
        answer, duration_line, debug_lines = output

        if duration_line is not None:
            msecs = float(duration_line[len("_duration:") :])

        if len(debug_lines) != 0:
            if no_debug:
                raise UnexpectedDebugLinesException(
                    f"unexpected debug lines in {submission.path()}:\n"
                    + "\n".join(debug_lines)
                )
            else:
                print("\n".join(debug_lines))
    else:
        answer = str(output)

    assert answer is not None
    parser = problem.parser()
    if parser is not None:
        answer = parser.parse(answer)
    if previous is not None and answer != previous.answer:
        raise DifferentAnswersException(
            f"""different answers day:{problem.day} part:{problem.part}
input: {input.path()}
{previous.submission.path()}: {previous.answer}
{submission.path()}: {answer}"""
        )
    return Result(problem, submission, input, answer, msecs)


def print_results(results: list[Result], print_time_dist: bool = False) -> None:
    results.sort(key=lambda x: x.duration)
    print(
        tabulate(
            [
                [
                    "  {color}{author}{end}  ".format(
                        color=(
                            BColor.BOLD
                            if result.submission.author == CONFIG.user
                            else BColor.GREEN
                        ),
                        author=result.submission.author,
                        end=BColor.ENDC,
                    ),
                    "  {color}{answer}{end}  ".format(
                        color=(
                            BColor.BOLD
                            if result.submission.author == CONFIG.user
                            else BColor.BLUE
                        ),
                        answer=result.answer,
                        end=BColor.ENDC,
                    ),
                    "  {color}{msecs:8.3f} ms{end}".format(
                        color=BColor.BOLD, msecs=result.duration, end=BColor.ENDC
                    ),
                    "  {color}{language}{end}".format(
                        color=(
                            BColor.BOLD
                            if result.submission.author == CONFIG.user
                            else ""
                        ),
                        language=result.submission.language,
                        end=BColor.ENDC,
                    ),
                    "  {color}{time_distribution}{end}".format(
                        color=(
                            BColor.BOLD
                            if result.submission.author == CONFIG.user
                            else ""
                        ),
                        time_distribution=get_time_distribution(result.all_durations),
                        end=BColor.ENDC,
                    )
                    if print_time_dist
                    else None,
                ]
                for result in results
            ]
        )
    )


def print_expanded_results(results_by_input: Mapping[str, list[Result]]) -> None:
    for input_author, submission_results in results_by_input.items():
        print("---------------------------------------------------")
        print(
            "On input from {yellow}{author}{end}".format(
                yellow=BColor.YELLOW, end=BColor.ENDC, author=input_author
            )
        )
        print("---------------------------------------------------")
        print_results(submission_results)


def print_restrict_results(results_by_author: Mapping[str, list[Result]]) -> None:
    print("---------------------------------------------------")
    print("On own inputs")
    print("---------------------------------------------------")
    results: list[Result] = []
    for _, results_by_input in results_by_author.items():
        for result in results_by_input:
            results.append(result)
    print_results(results)


def print_aggregated_header():
    print("---------------------------------------------------")
    print("Avg over all inputs")
    print("---------------------------------------------------")


def get_aggregated_results(
    problem: Problem,
    results_by_author: Mapping[str, list[Result]],
    print_time_dist: bool = False,
) -> list[Result]:
    results: list[Result] = []
    # Loop for all authors, get all the results they produced
    for author, results_by_input in results_by_author.items():
        res_by_language: dict[str, Result] = {}
        count_by_language: defaultdict[str, int] = defaultdict(int)
        durations_by_language: defaultdict[str, list[float]] = defaultdict(list)
        # The results can be made by different languages. Make a virtual result (storing total duration) by language
        for result in results_by_input:
            result_language = result.submission.language
            count_by_language[result_language] += 1
            # New language: make the virtual result
            if result_language not in res_by_language:
                res = Result(
                    problem,
                    Submission(problem, author, result_language, init_runnable=False),
                    None,
                    "-",
                    0,
                )
                res_by_language[result_language] = res
            assert result.input is not None
            # The author is on his own input, get his answer (split to allow author.x.lang on input author.txt)
            if author.split(".")[0] == result.input.author:
                res_by_language[result_language].answer = result.answer
                res_by_language[result_language].input = result.input
                res_by_language[result_language].submission = result.submission
            # Add up the duration of this result
            res_by_language[result_language].duration += result.duration
            durations_by_language[result_language].append(result.duration)
        # For each language of the author, make the average and store the final result
        for lang, res in res_by_language.items():
            if count_by_language[lang] > 0:
                res.duration /= count_by_language[lang]
            res.all_durations = durations_by_language[lang]
            results.append(res)
    return results


def print_day_header(problem: Problem) -> None:
    print("~" * 50)
    print(
        f"{BColor.RED}{BColor.BOLD}Running submissions for day {problem.day:02d}:{BColor.ENDC}"
    )


def print_part_header(problem: Problem):
    print(f"\n{BColor.MAGENTA}{BColor.BOLD}* part {problem.part}:{BColor.ENDC}")
