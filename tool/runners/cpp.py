import errno
import subprocess
import tempfile

from tool.runners.exceptions import CompilationError, RuntimeError
from tool.runners.wrapper import SubmissionWrapper


class SubmissionCpp(SubmissionWrapper):
    def __init__(self, file: str) -> None:
        SubmissionWrapper.__init__(self)
        tmp = tempfile.NamedTemporaryFile(prefix="aoc")
        tmp.close()
        compile_output = subprocess.check_output(
            [
                "clang-16",
                "-Wall",
                "-Wextra",
                "-O3",
                "-lstdc++",
                "-lm",
                "-std=c++20",
                "-o",
                tmp.name,
                file,
            ]
        ).decode()
        if compile_output:
            raise CompilationError(compile_output)
        self.executable = tmp.name

    def exec(self, input: str) -> str:
        try:
            return subprocess.check_output([self.executable, input]).decode()
        except OSError as e:
            if e.errno == errno.ENOENT:
                # executable not found
                raise CompilationError(e)
            else:
                # subprocess exited with another error
                raise RuntimeError(e)
