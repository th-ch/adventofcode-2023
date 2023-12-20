from tool.runners.python import SubmissionPy

from copy import deepcopy
from collections import defaultdict, deque

FLIP_FLOP, CONJUNCTION = "%", "&"
LOW, HIGH = "low", "high"

def push_button(modules):
    to_process = deque([("button", "broadcaster", LOW)])
    nb_low, nb_high = 1, 0
    while to_process:
        source, module, pulse = to_process.popleft()
        # print(f"{source} -{pulse}-> {module}")
        if modules[module]["type"] == FLIP_FLOP:
            if pulse == HIGH:
                continue
            modules[module]["on"] = not modules[module]["on"]
            new_pulse = HIGH if modules[module]["on"] else LOW
        elif modules[module]["type"] == CONJUNCTION:
            modules[module]["inputs"][source] = pulse
            new_pulse = LOW if all(p == HIGH for p in modules[module]["inputs"].values()) else HIGH
        else:
            # button or broadcaster
            new_pulse = pulse

        to_process.extend([(module, output, new_pulse) for output in modules[module]["outputs"]])
        if new_pulse == LOW:
            nb_low += len(modules[module]["outputs"])
        else:
            nb_high += len(modules[module]["outputs"])

    return nb_low, nb_high


def modules_equal(modules1, modules2):
    for module in modules1.keys():
        if modules1[module]["type"] == FLIP_FLOP and modules1[module]["on"] != modules2[module]["on"]:
            return False
        if modules1[module]["type"] == CONJUNCTION and modules1[module]["inputs"] != modules2[module]["inputs"]:
            return False
    return True

class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        modules = defaultdict(lambda: {"type": None, "outputs": []})
        modules["button"] = {"type": None, "outputs": ["broadcaster"]}
        for line in s.splitlines():
            module, outputs = line.split(" -> ")
            state = {"type": None}
            if module[0] == FLIP_FLOP:
                module = module[1:]
                state = {"type": FLIP_FLOP, "on": False}
            elif module[0] == CONJUNCTION:
                module = module[1:]
                state = {"type": CONJUNCTION, "inputs": {}}

            state.update({"outputs": outputs.split(", ")})
            modules[module] = state

        for module in modules.copy().keys():
            for output in modules[module]["outputs"]:
                if modules[output]["type"] == CONJUNCTION:
                    modules[output]["inputs"][module] = LOW

        original_modules = deepcopy(modules)
        i = 0
        TOTAL = 1000
        total_nb_low, total_nb_high = 0, 0
        while i < TOTAL:
            nb_low, nb_high = push_button(modules)
            total_nb_low += nb_low
            total_nb_high += nb_high
            i += 1
            if modules_equal(modules, original_modules):
                break

        nb_cycles = TOTAL // i
        return (nb_cycles * total_nb_low) * (nb_cycles * total_nb_high)



def test_th_ch():
    """
    Run `python -m pytest ./day-20/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            r"""
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
""".strip()
        )
        == 32000000
    )
    assert (
        ThChSubmission().run(
            r"""
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
""".strip()
        )
        == 11687500
    )
