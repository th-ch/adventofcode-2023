#include <chrono>
#include <deque>
#include <iostream>
#include <span>
#include <sstream>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <variant>
#include <vector>

static constexpr int kPresses = 1000;

struct ConjunctionState {
  std::unordered_map<int, bool> inputs;
  int count_low;
};

struct Module {
  std::variant<bool, ConjunctionState> state;
  std::vector<int> targets;
};

using Modules = std::array<Module, 26 * 26>;

int ToIndex(char c1, char c2) { return 26 * (c1 - 'a') + c2 - 'a'; }

struct State {
  Modules modules;
  std::vector<int> broadcaster;
};

void ParseLine(const std::string& line, State& state,
               std::vector<int>& module_indexes) {
  std::vector<int> targets;
  for (int t_pos = line.find('>') + 2; t_pos < line.size(); t_pos += 4) {
    targets.push_back(ToIndex(line[t_pos], line[t_pos + 1]));
  }
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
      std::swap(module.targets, targets);
      break;
    }
    default:
      throw std::invalid_argument("Invalid first character in ParseLine.");
  }
}

bool IsFlipFlop(const Module& module) {
  return std::holds_alternative<bool>(module.state);
}

bool& GetFlipFlopState(Module& module) { return std::get<bool>(module.state); }

bool IsConjunction(const Module& module) { return !IsFlipFlop(module); }

ConjunctionState& GetConjunctionState(Module& module) {
  return std::get<ConjunctionState>(module.state);
}

void InitializeConjunctions(State& state,
                            const std::vector<int>& module_indexes) {
  for (int from : module_indexes) {
    for (int to : state.modules[from].targets) {
      if (IsFlipFlop(state.modules[to])) continue;
      GetConjunctionState(state.modules[to]).inputs.insert({from, false});
      ++GetConjunctionState(state.modules[to]).count_low;
    }
  }
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

std::pair<int, int> ProcessPress(State& state) {
  std::deque<Pulse> pulses;
  for (int target : state.broadcaster) {
    pulses.push_back({false, -1, target});
  }
  int count_low = 0;
  int count_high = 0;
  while (!pulses.empty()) {
    Pulse pulse = pulses.front();
    pulses.pop_front();
    if (pulse.high) {
      ++count_high;
    } else {
      ++count_low;
    }
    ProcessPulse(state, pulse, pulses);
  }
  return {count_low, count_high};
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  State state;
  std::vector<int> module_indexes;
  for (std::string line; std::getline(iss, line);) {
    ParseLine(line, state, module_indexes);
  }
  InitializeConjunctions(state, module_indexes);
  int count_low = 0;
  int count_high = 0;
  for (int press = 0; press < kPresses; ++press) {
    auto [low, high] = ProcessPress(state);
    count_low += low + 1;
    count_high += high;
  }
  return std::to_string(count_low * count_high);
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
