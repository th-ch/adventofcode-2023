"""
Wrapper class handling the communication between the main python process and
the funky language subprocesses.
"""

from tool.runners.python import WithDebugStack


class SubmissionWrapper(WithDebugStack):
    def __init__(self) -> None:
        super().__init__()

    # Method that every class implementing SubmssionWrapper should override
    def exec(self, input: str) -> str:
        raise NotImplementedError

    def run(self, input: str) -> tuple[str | None, str | None, list[str]]:
        stdout = self.exec(input)
        lines = stdout.split("\n")[:-1]

        duration_line: str | None = None
        parse = False
        for line in lines:
            if line.startswith("_duration:"):
                duration_line = line
            if line.startswith("_parse"):
                parse = True
        lines = [
            line
            for line in lines
            if not (line.startswith("_duration:") or line.startswith("_parse"))
        ]

        if len(lines) == 0:
            return None, duration_line, []
        if parse:
            return "\n".join(lines), duration_line, []
        return lines[-1], duration_line, lines[:-1]
