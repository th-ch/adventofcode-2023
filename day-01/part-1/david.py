from tool.runners.python import SubmissionPy


class DavidSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        lines = s.split("\n")
        result = 0
        for line in lines:
            digits = [int(x) for x in line if x.isdigit()]
            result += (digits[0]*10 + digits[-1])
        return result
