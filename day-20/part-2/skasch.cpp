#include <chrono>
#include <deque>
#include <iostream>
#include <numeric>
#include <span>
#include <sstream>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <variant>
#include <vector>

struct ConjunctionState {
  std::unordered_map<int, bool> inputs;
  int count_low;
};

struct Module {
  std::variant<bool, ConjunctionState> state;
  std::vector<int> targets;
};

using Modules = std::array<Module, 26 * 26>;

constexpr int ToIndex(char c1, char c2) { return 26 * (c1 - 'a') + c2 - 'a'; }

static constexpr int kRx = ToIndex('r', 'x');

struct State {
  Modules modules;
  std::vector<int> broadcaster;
};

int ParseLine(const std::string& line, State& state,
              std::vector<int>& module_indexes) {
  std::vector<int> targets;
  bool is_rx_inverter = false;
  for (int t_pos = line.find('>') + 2; t_pos < line.size(); t_pos += 4) {
    targets.push_back(ToIndex(line[t_pos], line[t_pos + 1]));
    if (targets.back() == kRx) is_rx_inverter = true;
  }
  int rx_inverter = 0;
  switch (line[0]) {
    case 'b': {
      std::swap(state.broadcaster, targets);
      break;
    }
    case '%': {
      module_indexes.push_back(ToIndex(line[1], line[2]));
      Module& module = state.modules[module_indexes.back()];
      module.state = false;
      std::swap(module.targets, targets);
      break;
    }
    case '&': {
      module_indexes.push_back(ToIndex(line[1], line[2]));
      Module& module = state.modules[module_indexes.back()];
      module.state = ConjunctionState();
      if (is_rx_inverter) rx_inverter = module_indexes.back();
      std::swap(module.targets, targets);
      break;
    }
    default:
      throw std::invalid_argument("Invalid first character in ParseLine.");
  }
  return rx_inverter;
}

bool IsFlipFlop(const Module& module) {
  return std::holds_alternative<bool>(module.state);
}

bool& GetFlipFlopState(Module& module) { return std::get<bool>(module.state); }

bool IsConjunction(const Module& module) { return !IsFlipFlop(module); }

ConjunctionState& GetConjunctionState(Module& module) {
  return std::get<ConjunctionState>(module.state);
}

std::unordered_set<int> InitializeConjunctions(
    State& state, const std::vector<int>& module_indexes, int rx_inverter) {
  std::unordered_set<int> rx_inputs;
  for (int from : module_indexes) {
    for (int to : state.modules[from].targets) {
      if (IsFlipFlop(state.modules[to])) continue;
      GetConjunctionState(state.modules[to]).inputs.insert({from, false});
      ++GetConjunctionState(state.modules[to]).count_low;
      if (to == rx_inverter) rx_inputs.insert(from);
    }
  }
  return rx_inputs;
}

struct Pulse {
  bool high;
  int source;
  int target;
};

void ProcessPulse(State& state, const Pulse& pulse, std::deque<Pulse>& pulses) {
  Module& module = state.modules[pulse.target];
  if (IsFlipFlop(module)) {
    if (pulse.high) return;
    bool& state = GetFlipFlopState(module);
    state = !state;
    for (int target : module.targets) {
      pulses.push_back({state, pulse.target, target});
    }
  } else {
    ConjunctionState& state = GetConjunctionState(module);
    state.count_low -= pulse.high - state.inputs[pulse.source];
    state.inputs[pulse.source] = pulse.high;
    for (int target : module.targets) {
      pulses.push_back({state.count_low != 0, pulse.target, target});
    }
  }
}

std::unordered_set<int> ProcessPress(State& state,
                                     const std::unordered_set<int>& rx_inputs) {
  std::deque<Pulse> pulses;
  std::unordered_set<int> outputs;
  for (int target : state.broadcaster) {
    pulses.push_back({false, -1, target});
  }
  while (!pulses.empty()) {
    Pulse pulse = pulses.front();
    pulses.pop_front();
    if (!pulse.high && rx_inputs.contains(pulse.target))
      outputs.insert(pulse.target);
    ProcessPulse(state, pulse, pulses);
  }
  return outputs;
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  State state;
  std::vector<int> module_indexes;
  int rx_inverter = 0;
  for (std::string line; std::getline(iss, line);) {
    if (int v = ParseLine(line, state, module_indexes); v != 0) rx_inverter = v;
  }
  std::unordered_set<int> rx_inputs =
      InitializeConjunctions(state, module_indexes, rx_inverter);
  std::unordered_map<int, int> periods;
  for (int press = 1;; ++press) {
    for (int output : ProcessPress(state, rx_inputs)) {
      if (periods.contains(output)) continue;
      periods[output] = press;
    }
    if (periods.size() == rx_inputs.size()) {
      std::int64_t result = 1;
      for (const auto& [_, period] : periods) {
        result = std::lcm(period, result);
      }
      return std::to_string(result);
    }
  }
}

int main(int argc, char* argv[]) {
  if (argc < 2) {
    std::cout << "Missing one argument" << std::endl;
    exit(1);
  }
  auto args = std::span(argv, static_cast<size_t>(argc));

  auto start = std::chrono::high_resolution_clock::now();
  auto answer = Run(args[1]);
  auto end = std::chrono::high_resolution_clock::now();

  std::cout << "_duration:"
            << std::chrono::duration<float, std::milli>(end - start).count()
            << "\n";

  std::cout << answer << "\n";
  return 0;
}
