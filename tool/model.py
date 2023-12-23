#!/usr/bin/env python3

# stdlib
import os.path

# project
from tool.parser import Parser
from tool.runners import ext_by_language, load_submission_runnable
from tool.utils import load_subclass


class Problem(object):
    @staticmethod
    def day_to_path(day: int) -> str:
        return f"day-{day:02d}"

    def __init__(self, day: int, part: int):
        self.day = day
        self.part = part
        self._parser_cache: tuple[bool, Parser | None] = (False, None)

    def __repr__(self):
        return f"Problem{{day-{self.day:02d}, part-{self.part}}}"

    def day_path(self):
        return Problem.day_to_path(self.day)

    def path(self):
        return os.path.join(self.day_path(), f"part-{self.part}")

    def parser(self):
        is_cached, parser = self._parser_cache
        if is_cached:
            return parser

        path = os.path.join(self.day_path(), f"parser-{self.part}.py")
        if not os.path.exists(path):
            self._parser_cache = (True, None)
            return None

        parser = load_subclass(path, Parser)
        self._parser_cache = (True, parser)
        return parser


class Submission(object):
    def __init__(
        self,
        problem: Problem,
        author: str,
        language: str,
        content: str | None = None,
        init_runnable: bool = True,
    ) -> None:
        self.problem = problem
        self.author = author
        self.language = language
        self.content = content
        self.runnable = (
            load_submission_runnable(self.path(), language) if init_runnable else None
        )

    def __repr__(self):
        return f"Submission{{{self.problem}, by {self.author}, in {self.language}}}"

    def path(self):
        return os.path.join(
            self.problem.path(), f"{self.author}{ext_by_language(self.language)}"
        )


class Input(object):
    def __init__(self, problem: Problem, author: str, content: str) -> None:
        self.problem = problem
        self.author = author
        self.content = content

    def __repr__(self):
        return f"Input{{{self.problem}, by {self.author}, size {len(self.content)}}}"

    def path(self):
        return os.path.join(self.problem.day_path(), "input", self.author + ".txt")


class Result(object):
    def __init__(
        self,
        problem: Problem,
        submission: Submission,
        input: Input | None,
        answer: str,
        duration: float,
    ) -> None:
        self.problem = problem
        self.submission = submission
        self.input = input
        self.answer = answer
        self.duration = duration
        self.all_durations: list[float] = []

    def __repr__(self):
        return (
            f"Result{{{self.problem}, {self.submission}, {self.input}, {self.answer}}}"
        )
