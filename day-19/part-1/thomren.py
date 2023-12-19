import inspect
from typing import NamedTuple
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        workflows, pieces = self.parse_input(s)

        res = 0
        for piece in pieces:
            if is_piece_approved(piece, workflows):
                res += sum(piece.values())
        return res

    @staticmethod
    def parse_input(s):
        workflows_str, pieces_str = s.split("\n\n")

        workflows = {}
        for workflow_str in workflows_str.splitlines():
            name, rules_str = workflow_str[:-1].split("{")
            rules = []
            for rule_str in rules_str.split(","):
                test = None
                if ":" in rule_str:
                    test_str, result = rule_str.split(":")
                    key, value = test_str[0], int(test_str[2:])
                    if test_str[1] == "<":
                        test = lambda piece, key=key, value=value: (piece[key] < value)
                    elif test_str[1] == ">":
                        test = lambda piece, key=key, value=value: (piece[key] > value)
                    else:
                        raise ValueError(f"Unknown test {test}")
                    rules.append((test, result))
                else:
                    test = lambda _: True
                    rules.append((test, rule_str))
            workflows[name] = rules

        pieces = []
        for piece_str in pieces_str.splitlines():
            piece = {}
            for var_str in piece_str[1:-1].split(","):
                var, value = var_str.split("=")
                piece[var] = int(value)
            pieces.append(piece)

        return workflows, pieces


def is_piece_approved(piece, workflows, start="in"):
    for test, result in workflows[start]:
        if test(piece):
            if result in ("A", "R"):
                return result == "A"
            return is_piece_approved(piece, workflows, result)
    raise ValueError("invalid workflows")


def test_thomren():
    """
    Run `python -m pytest ./day-19/part-1/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
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
