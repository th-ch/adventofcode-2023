from collections import deque
from dataclasses import dataclass
from enum import Enum
from math import lcm
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        modules = {}
        for line in s.splitlines():
            name, outputs = line.split(" -> ")
            outputs = outputs.split(", ")
            if name[0] == "%":
                module = FlipFlop(name[1:], outputs)
            elif name[0] == "&":
                module = Conjunction(name[1:], outputs, {})
            elif name == "broadcaster":
                module = Broadcaster(name, outputs)
            modules[module.name] = module

        conjunctions = [
            module.name
            for module in modules.values()
            if isinstance(module, Conjunction)
        ]
        for module in modules:
            for conjunction_name in conjunctions:
                if conjunction_name in modules[module].outputs:
                    modules[conjunction_name].inputs_last_signal[module] = Signal.LOW

        rx_input = next(
            module.name for module in modules.values() if "rx" in module.outputs
        )
        rx_inputs = [
            module.name for module in modules.values() if rx_input in module.outputs
        ]
        rx_inputs_pushes = {}

        signals_queue = deque()
        pushes = 0
        while pushes <= 2**12:
            pushes += 1
            signals_queue.append(("button", "broadcaster", Signal.LOW))
            while signals_queue:
                input_name, module_name, signal = signals_queue.popleft()

                if input_name in rx_inputs and signal == Signal.HIGH:
                    rx_inputs_pushes[input_name] = pushes
                    if len(rx_inputs_pushes) == len(rx_inputs):
                        return lcm(*rx_inputs_pushes.values())

                module = modules.get(module_name)
                if isinstance(module, FlipFlop):
                    if signal == Signal.LOW:
                        module.on = not module.on
                        for output in module.outputs:
                            signals_queue.append(
                                (
                                    module.name,
                                    output,
                                    Signal.HIGH if module.on else Signal.LOW,
                                )
                            )
                elif isinstance(module, Conjunction):
                    module.inputs_last_signal[input_name] = signal
                    for output in module.outputs:
                        signals_queue.append(
                            (
                                module.name,
                                output,
                                Signal.LOW
                                if all(
                                    s == Signal.HIGH
                                    for s in module.inputs_last_signal.values()
                                )
                                else Signal.HIGH,
                            )
                        )
                elif isinstance(module, Broadcaster):
                    for output in module.outputs:
                        signals_queue.append((module.name, output, signal))

        return lcm(*rx_inputs_pushes.values())


class Signal(Enum):
    LOW = 0
    HIGH = 1


@dataclass
class FlipFlop:
    name: str
    outputs: list[str]
    on: bool = False


@dataclass
class Conjunction:
    name: str
    outputs: list[str]
    inputs_last_signal: dict[str, Signal]


@dataclass
class Broadcaster:
    name: str
    outputs: list[str]
