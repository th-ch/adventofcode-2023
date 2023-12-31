#!/usr/bin/env python3

# project
from tool.runners.bash import SubmissionBash
from tool.runners.c import SubmissionC
from tool.runners.cpp import SubmissionCpp
from tool.runners.cs import SubmissionCs
from tool.runners.cython_aoc import SubmissionPyx
from tool.runners.deno import SubmissionDeno
from tool.runners.go import SubmissionGo
from tool.runners.intcode import SubmissionIntcode
from tool.runners.java import SubmissionJava
from tool.runners.julia import SubmissionJulia
from tool.runners.nim import SubmissionNim
from tool.runners.php import SubmissionPHP
from tool.runners.python import SubmissionPy
from tool.runners.ruby import SubmissionRb
from tool.runners.rust import SubmissionRs

# from tool.runners.vlang import SubmissionV
from tool.runners.wrapper import SubmissionWrapper
from tool.runners.zig import SubmissionZig
from tool.utils import load_subclass

TOOL_BY_LANGUAGE = {
    "c": "gcc",
    "cpp": "g++",
    "cs": "dotnet",
    "go": "go",
    "intcode": "python3",
    "java": "java",
    "js": "deno",
    "ts": "deno",
    "nim": "nim",
    "ml": "dune",
    "php": "php",
    "py": "python3",
    "pyx": "cython",
    "rb": "ruby",
    "rs": "cargo",
    "sh": "bash",
    "jl": "julia",
    # "v": "v",
    "zig": "zig",
}
LANGUAGES = list(TOOL_BY_LANGUAGE.keys())


def ext_by_language(lang: str) -> str:
    return "." + lang


def load_submission_runnable(
    path: str, language: str
) -> SubmissionPy | SubmissionWrapper | None:
    if language not in LANGUAGES:
        return None
    if language == "py":
        return load_subclass(path, SubmissionPy, exclude=[SubmissionWrapper])
    elif language == "pyx":
        return SubmissionPyx(path)
    elif language == "c":
        return SubmissionC(path)
    elif language == "cpp":
        return SubmissionCpp(path)
    elif language == "cs":
        return SubmissionCs(path)
    elif language == "go":
        return SubmissionGo(path)
    elif language == "intcode":
        return SubmissionIntcode(path)
    elif language == "java":
        return SubmissionJava(path)
    elif language == "js" or language == "ts":
        return SubmissionDeno(path)
    elif language == "php":
        return SubmissionPHP(path)
    elif language == "rb":
        return SubmissionRb(path)
    elif language == "rs":
        return SubmissionRs(path)
    elif language == "sh":
        return SubmissionBash(path)
    elif language == "jl":
        return SubmissionJulia(path)
    elif language == "nim":
        return SubmissionNim(path)
    # elif language == "v":
    #     return SubmissionV(path)
    elif language == "zig":
        return SubmissionZig(path)
