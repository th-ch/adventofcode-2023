from tool.runners.python import SubmissionPy

from typing import Iterator
from collections import defaultdict

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

    def _get_adjacent_numbers(self, grid, pos, number_cells):
        result = []
        i0, j0 = pos
        for i in (i0-1, i0, i0+1):
            for start, end in number_cells[i]:
                if j0 >= start - 1 and j0 <= end:
                    result.append(int(grid[i][start:end]))
        return result
            
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        grid = s.split("\n")

        number_cells = defaultdict(list)
        for i, line in enumerate(grid):
            for start, end in self._scan_line(line):
                number_cells[i].append((start, end))

        result = 0
        for i, line in enumerate(grid):
            for j, x in enumerate(line):
                if x != '*':
                    continue

                adjacent_numbers = self._get_adjacent_numbers(grid, (i,j), number_cells)
                if len(adjacent_numbers) != 2:
                    continue
                
                result += adjacent_numbers[0] * adjacent_numbers[1]
        
        return result
                





