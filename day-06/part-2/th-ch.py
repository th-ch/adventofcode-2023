from tool.runners.python import SubmissionPy

import math

class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        lines = s.splitlines()
        time = int(lines[0][11:].replace(" ", ""))
        distance = int(lines[1][11:].replace(" ", ""))

        ####### Naive solution
        # hold = 0
        # while True:
        #     d = (time - hold) * hold
        #     if d > distance:
        #         break
        #     hold += 1
        # start = hold

        # hold = time
        # while True:
        #     d = (time - hold) * hold
        #     if d > distance:
        #         break
        #     hold -= 1

        # return hold - start + 1

        ####### Equation solution
        # (time - hold) * hold > distance
        # hold^2 - time*hold + distance < 0
        start = math.ceil((time-math.sqrt(time**2-4*distance))/2)
        end = math.floor((time+math.sqrt(time**2-4*distance))/2)

        return int(end - start + 1)



def test_th_ch():
    """
    Run `python -m pytest ./day-06/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
Time:      7  15   30
Distance:  9  40  200
""".strip()
        )
        == 71503
    )
