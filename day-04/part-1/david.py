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
            winning_part = line[INDEX_START+1:INDEX_SEPERATOR]
            winning_numbers = set(int(x) for x in winning_part.strip().split())
            candidate_part = line[INDEX_SEPERATOR+1:]
            candidate_numbers = set(int(x) for x in candidate_part.strip().split())
            count_winning = len(winning_numbers & candidate_numbers)

            if count_winning > 0:
                result += (1<<(count_winning-1))
        return result
