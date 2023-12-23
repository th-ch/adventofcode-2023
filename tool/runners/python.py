#!/usr/bin/env python3

from inspect import getframeinfo, stack


class WithDebugStack:
    def __init__(self) -> None:
        self.debug_stack: list[str] = []

    def debug(self, message: str) -> None:
        caller = getframeinfo(stack()[1][0])
        self.debug_stack.append(f"{caller.filename}:{caller.lineno} - {message}")

    def get_debug_stack(self) -> list[str]:
        return self.debug_stack

    def cleanup(self) -> None:
        pass


class SubmissionPy(WithDebugStack):
    def __init__(self) -> None:
        super().__init__()

    # Method that every class implementing Submission should override
    def run(self, input: str) -> str:
        raise NotImplementedError
