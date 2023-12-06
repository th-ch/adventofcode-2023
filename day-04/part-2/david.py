from tool.runners.python import SubmissionPy


class DavidSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        result = 0
        lines = s.split("\n")
        INDEX_START = lines[0].index(":")
        INDEX_SEPERATOR = lines[0].index("|")
        copies = [1]*len(lines)
        for i, line in enumerate(lines):
            left_part = line[INDEX_START+1:INDEX_SEPERATOR]
            left_numbers = set(int(x) for x in left_part.split())
            right_part = line[INDEX_SEPERATOR+1:]
            right_numbers = set(int(x) for x in right_part.split())
            
            count_winners = len(left_numbers & right_numbers)
            for k in range(count_winners):
                copies[i+1+k] += copies[i]
        return sum(copies)
