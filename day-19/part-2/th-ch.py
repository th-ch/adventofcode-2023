from tool.runners.python import SubmissionPy

from collections import deque


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        workflows_str, ratings_str = s.split("\n\n")
        ratings = []
        for rating_str in ratings_str.split("\n"):
            rating_str = rating_str[1:-1]
            rating = {}
            for r in rating_str.split(","):
                k, v = r.split("=")
                rating[k] = int(v)
            ratings.append(rating)

        workflows = {}
        for workflow in workflows_str.split("\n"):
            name, instructions = workflow.split("{")
            instructions = instructions[:-1]
            workflows[name] = []
            for instruction in instructions.split(","):
                splitted = instruction.split(":")
                if len(splitted) == 1:
                    action = splitted[0]
                    condition = (None, -1)
                else:
                    condition, action = splitted
                    criterion = condition[0]
                    threshold = int(condition[2:])
                    condition = (condition[1], threshold)

                workflows[name].append((criterion, condition, action))

        to_process = deque()
        to_process.append(("in", {"x": (1, 4000), "m": (1, 4000), "a": (1, 4000), "s": (1, 4000)}))
        accepted = []
        while to_process:
            output, rating_range = to_process.popleft()
            if output == "R":
                continue
            elif output == "A":
                accepted.append(rating_range)
                continue

            if any(rating_range[criterion][0] > rating_range[criterion][1] for criterion in rating_range):
                continue

            for criterion, (operator, threshold), action in workflows[output]:
                if operator == "<":
                    first_range = (rating_range[criterion][0], threshold-1)
                    second_range = (threshold, rating_range[criterion][1])

                    new_range_rating = rating_range.copy()
                    new_range_rating[criterion] = first_range
                    to_process.append((action, new_range_rating))
                    rating_range[criterion] = second_range
                elif operator == ">":
                    first_range = (rating_range[criterion][0], threshold)
                    second_range = (threshold+1, rating_range[criterion][1])

                    new_range_rating = rating_range.copy()
                    new_range_rating[criterion] = second_range
                    to_process.append((action, new_range_rating))
                    rating_range[criterion] = first_range
                else:
                    to_process.append((action, rating_range))

        result = 0
        for rating_range in accepted:
            nb = 1
            for criterion in rating_range:
                nb *= (rating_range[criterion][1] - rating_range[criterion][0] + 1)
            result += nb
        return result


def test_th_ch():
    """
    Run `python -m pytest ./day-19/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
""".strip()
        )
        == 167409079868000
    )
