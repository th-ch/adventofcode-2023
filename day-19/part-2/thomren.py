from collections import deque
from copy import deepcopy
from dataclasses import dataclass
from functools import reduce
import inspect
from typing import NamedTuple
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        workflows = self.parse_workflows(s)

        stack = [
            ("in", {"x": [1, 4000], "m": [1, 4000], "a": [1, 4000], "s": [1, 4000]})
        ]
        res = 0
        while stack:
            name, ranges = stack.pop()
            for rule in workflows[name]:
                pass_ranges = deepcopy(ranges)
                fail_ranges = {"x": [], "m": [], "a": [], "s": []}
                if rule.op == ">":
                    pass_ranges = deepcopy(ranges)
                    pass_ranges[rule.key][0] = max(
                        pass_ranges[rule.key][0], rule.value + 1
                    )
                    fail_ranges = deepcopy(ranges)
                    fail_ranges[rule.key][1] = min(fail_ranges[rule.key][1], rule.value)
                elif rule.op == "<":
                    pass_ranges = deepcopy(ranges)
                    pass_ranges[rule.key][1] = min(
                        pass_ranges[rule.key][1], rule.value - 1
                    )
                    fail_ranges = deepcopy(ranges)
                    fail_ranges[rule.key][0] = max(fail_ranges[rule.key][0], rule.value)
                elif rule.op is None:
                    pass_ranges = deepcopy(ranges)
                    fail_ranges = {"x": [], "m": [], "a": [], "s": []}

                if rule.result == "A":
                    res += reduce(
                        lambda x, y: x * y,
                        (r[1] - r[0] + 1 for r in pass_ranges.values()),
                        1,
                    )
                elif rule.result != "R":
                    stack.append((rule.result, pass_ranges))
                ranges = fail_ranges
        return res

    @staticmethod
    def parse_workflows(s):
        workflows_str, _ = s.split("\n\n")

        workflows = {}
        for workflow_str in workflows_str.splitlines():
            name, rules_str = workflow_str[:-1].split("{")
            rules = []
            for rule_str in rules_str.split(","):
                if ":" in rule_str:
                    rule_str, result = rule_str.split(":")
                    key, value = rule_str[0], int(rule_str[2:])
                    assert rule_str[1] in ("<", ">")
                    rule = Rule(key, value, rule_str[1], result)
                    rules.append(rule)
                else:
                    rule = Rule(None, None, None, rule_str)
                    rules.append(rule)
            workflows[name] = rules

        return workflows


@dataclass(frozen=True)
class Rule:
    key: str
    value: int
    op: str
    result: str


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
        == 167409079868000
    )
