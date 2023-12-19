from tool.runners.python import SubmissionPy


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
                    condition = lambda rating, threshold: True
                else:
                    condition, action = splitted
                    criterion = condition[0]
                    threshold = int(condition[2:])
                    if condition[1] == "<":
                        condition = lambda rating, threshold: rating < threshold
                    elif condition[1] == ">":
                        condition = lambda rating, threshold: rating > threshold

                workflows[name].append((criterion, condition, threshold, action))

        result = 0
        for rating in ratings:
            workflow = "in"
            output = None
            while output not in ["A", "R"]:
                for criterion, condition, threshold, action in workflows[workflow]:
                    if condition(rating[criterion], threshold):
                        output = action
                        break
                workflow = output

            if output == "A":
                result += sum(rating.values())

        return result




def test_th_ch():
    """
    Run `python -m pytest ./day-19/part-1/th-ch.py` to test the submission.
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
        == 19114
    )
