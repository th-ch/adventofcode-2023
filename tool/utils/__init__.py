#!/usr/bin/env python3

# stdlib
import inspect
import os.path
from importlib.machinery import SourceFileLoader
from typing import Type, TypeVar

_orig_dir = os.path.dirname(os.path.realpath(__file__))


class BColor:
    """
    BColor printable colors in terminal
    """

    RED = "\033[91m"
    GREEN = "\033[92m"
    YELLOW = "\033[93m"
    BLUE = "\033[94m"
    MAGENTA = "\033[95m"
    ENDC = "\033[0m"
    BOLD = "\033[1m"
    UNDERLINE = "\033[4m"


def mkdirp(path: str) -> None:
    """
    Create directory if it does not exist already.
    path : str name of the directory to be created
    """
    if not os.path.exists(path):
        os.makedirs(path)


def to_ints(strs: list[str] | None) -> list[int] | None:
    if strs is None:
        return None
    return [int(s) for s in strs]


def resolve_path(*path: str):
    """Resolve any path based on the project root.
    resolve_path('foo', 'bar') will give an absolute path to your_project_directory/foo/bar
    If the path is already absolute, it will stay absolute
    """
    return os.path.abspath(os.path.join(_orig_dir, "..", "..", *path))


T = TypeVar("T")


def load_subclass(
    path: str, base_class: Type[T], exclude: list[Type[T]] | None = None
) -> T | None:
    exclude = list(exclude) if exclude else []
    exclude += [base_class]
    submission_module = SourceFileLoader(f"loader_{path}", path).load_module()
    classes = inspect.getmembers(submission_module, inspect.isclass)
    for _, cls in classes:
        if issubclass(cls, base_class) and cls not in exclude:
            return cls()
