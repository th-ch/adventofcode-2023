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
        for line in lines:
            left_part = line[INDEX_START+1:INDEX_SEPERATOR]
            left_numbers = set(int(x) for x in left_part.split())
            right_part = line[INDEX_SEPERATOR+1:]
            right_numbers = set(int(x) for x in right_part.split())
            
            count_winners = len(left_numbers & right_numbers)

            if count_winners > 0:
                result += (1<<(count_winners-1))
        return result
