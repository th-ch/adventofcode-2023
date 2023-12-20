from tool.runners.python import SubmissionPy

from collections import defaultdict, deque
from math import lcm

FLIP_FLOP, CONJUNCTION = "%", "&"
LOW, HIGH = "low", "high"

def push_button(modules, module_to_look_for):
    to_process = deque([("button", "broadcaster", LOW)])
    module_is_in_low_state = False
    while to_process:
        source, module, pulse = to_process.popleft()
        if module == module_to_look_for and pulse == LOW:
            module_is_in_low_state = True
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

    return module_is_in_low_state


# rx is connected to one CONJUNCTION module which so must be LOW pulse
# So all of rx grandparents must be in HIGH pulse
# All rx grandparents have a cycle, so computing the LCM of all cycles gives the solution
class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        modules = defaultdict(lambda: {"type": None, "outputs": []})
        rx_parent = None
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
                if "rx" in outputs:
                    rx_parent = module

            state.update({"outputs": outputs.split(", ")})
            modules[module] = state

        for module in modules.copy().keys():
            for output in modules[module]["outputs"]:
                if modules[output]["type"] == CONJUNCTION:
                    modules[output]["inputs"][module] = LOW

        rx_grandparents = {module: None  for module in modules if rx_parent in modules[module]["outputs"]}
        for grandparent in rx_grandparents:
            i = 0
            iterations_module_in_low_state = []
            while True:
                module_is_in_low_state = push_button(modules, module_to_look_for=grandparent)
                if module_is_in_low_state:
                    iterations_module_in_low_state.append(i)
                    if len(iterations_module_in_low_state) >= 2:
                        cycle_size = iterations_module_in_low_state[1] - iterations_module_in_low_state[0]
                        rx_grandparents[grandparent] = cycle_size
                        break

                i += 1

        return lcm(*rx_grandparents.values())
