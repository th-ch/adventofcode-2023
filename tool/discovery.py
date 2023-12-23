#!/usr/bin/env python3

# stdlib
import glob
import os.path
from os import walk
from shutil import which

# project
from tool.model import Input, Problem, Submission
from tool.runners import LANGUAGES, TOOL_BY_LANGUAGE

_DAY_PATH_PATTERN = "day-[0-9]*"
_PART_PATH_PATTERN = "part-[0-9]*"
supported_languages = []
RANDOM_LANGUAGE = "random"


def get_accepted_languages_references() -> list[str]:
    # We add random to permit this possibility in the CLI
    return get_supported_languages() + [RANDOM_LANGUAGE]


def get_supported_languages() -> list[str]:
    global supported_languages
    if not supported_languages:
        supported_languages = [
            language
            for language in LANGUAGES
            if which(TOOL_BY_LANGUAGE[language]) is not None
        ]
    return supported_languages


def get_latest_problem() -> Problem | None:
    days = get_all_days()
    if not days:
        return
    latest_day = days[-1]
    parts = get_parts_for_day(latest_day)
    if not parts:
        return
    latest_part = parts[-1]
    return Problem(latest_day, latest_part)


def get_all_problems(days: list[int] | None = None) -> list[Problem]:
    if days is None:
        days = get_all_days()
    problems: list[Problem] = []
    for day in days:
        for part in get_parts_for_day(day):
            problems.append(Problem(day, part))

    return problems


def get_all_days() -> list[int]:
    return sorted(int(path[-2:]) for path in glob.glob(_DAY_PATH_PATTERN))


def get_parts_for_day(day: int) -> list[int]:
    return sorted(
        int(path[-1])
        for path in glob.glob(
            os.path.join(Problem.day_to_path(day), _PART_PATH_PATTERN)
        )
    )


def get_days_for_part(part: int) -> list[int]:
    return sorted(
        int(path[4:6])
        for path in glob.glob(os.path.join(_DAY_PATH_PATTERN, f"part-{part}"))
    )


def get_problems(
    days: list[int] | None, parts: list[int] | None, all_days_parts: bool = False
) -> list[Problem]:
    problems = []
    if all_days_parts:
        problems = get_all_problems()
    elif days is not None and parts is not None:
        problems = [
            problem for problem in get_all_problems(days) if problem.part in parts
        ]
    elif days is not None:
        problems = get_all_problems(days)
    elif parts is not None:
        latest = get_latest_problem()
        if not latest:
            return []
        problems = [
            Problem(latest.day, part)
            for part in get_parts_for_day(latest.day)
            if part in parts
        ]
    else:
        latest = get_latest_problem()
        if latest:
            problems = [
                Problem(latest.day, part) for part in get_parts_for_day(latest.day)
            ]
    return sorted(problems, key=lambda p: (p.day, p.part))


def get_submissions(
    problem: Problem,
    authors: list[str] | None = None,
    ignored_authors: list[str] | None = None,
    languages: list[str] | None = None,
    force: bool = False,
) -> list[Submission]:
    if languages is None:
        if force:
            languages = LANGUAGES
        else:
            languages = get_supported_languages()
    elif not force:
        languages = [
            language for language in languages if language in get_supported_languages()
        ]

    extensions = set(languages)

    submissions: list[Submission] = []
    for _, _, files in walk(problem.path()):
        for filename in files:
            submission, ext = filename.split(".", 1)
            author = os.path.basename(submission)
            if (ext not in extensions) or filename.endswith("_test.go"):
                continue
            if ignored_authors is not None and author in ignored_authors:
                continue
            if authors is not None and author not in authors:
                continue
            submissions.append(Submission(problem, author, ext))
        break  # stop at depth 1
    return submissions


def get_inputs(problem: Problem) -> list[Input]:
    inputs_path = os.path.join(problem.day_path(), "input")
    if not os.path.exists(inputs_path):
        return []

    inputs: list[Input] = []
    for input_file in glob.glob(os.path.join(inputs_path, "*.txt")):
        author = os.path.splitext(os.path.basename(input_file))[0].lower()
        with open(input_file, "r") as content_file:
            inputs.append(Input(problem, author, content_file.read().rstrip()))
    return inputs
