from tool.runners.python import SubmissionPy

from typing import Iterator

class DavidSubmission(SubmissionPy):
    def _scan_line(self, line: str) -> Iterator[tuple[int, int]]:
        start, end = None, None
        for i in range(len(line)):
            if not line[i].isdigit():
                if start is not None:
                    end = i
                    yield (start, end)
                    start = None
            else:
                if start is None:
                    start = i
        if start is not None:
            end = len(line)
            yield (start, end)

    def _is_symbol(self, x: str):
        return not (x.isdigit() or x == ".")

    def _has_adjacent_symbol(self, grid, i0, start, end) -> bool:
        n, m = len(grid), len(grid[0])
        for i in {max(0,i0-1), min(n-1,i0+1), i0}:
            if any(self._is_symbol(grid[i][j]) for j in range(max(0, start-1), min(m, end+1))):
                return True
            
        return False

    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        grid = s.split("\n")

        result = 0
        for i, line in enumerate(grid):
            for start, end in self._scan_line(line):
                if self._has_adjacent_symbol(grid, i, start, end):
                    result += int(line[start:end])

        return result
