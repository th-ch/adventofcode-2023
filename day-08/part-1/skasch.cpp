#include <chrono>
#include <iostream>
#include <span>
#include <sstream>
#include <string>

static std::array<std::pair<int, int>, 26 * 26 * 26> kGraph;

int GetValue(const std::string& node) {
  return (node[0] - 'A') + 26 * (node[1] - 'A') + 26 * 26 * (node[2] - 'A');
}

static constexpr int kStartNode = 0;
static constexpr int kEndNode = 26 * 26 * 26 - 1;

void UpdateGraph(const std::string& line) {
  kGraph[GetValue(line.substr(0, 3))] = {GetValue(line.substr(7, 3)),
                                         GetValue(line.substr(12, 3))};
}

int Navigate(const std::string& directions, int first_node, int last_node) {
  int directions_index = 0;
  int node = first_node;
  int steps = 0;
  for (; node != last_node; ++steps) {
    if (directions[directions_index] == 'L') {
      node = kGraph[node].first;
    } else {
      node = kGraph[node].second;
    }
    ++directions_index;
    if (directions_index == directions.size()) directions_index = 0;
  }
  return steps;
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  std::string directions;
  std::getline(iss, directions);
  std::string line;
  std::getline(iss, line);
  while (std::getline(iss, line)) {
    UpdateGraph(line);
  }
  int steps = Navigate(directions, kStartNode, kEndNode);
  return std::to_string(steps);
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
