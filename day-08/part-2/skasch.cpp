#include <chrono>
#include <cstdint>
#include <iostream>
#include <numeric>
#include <span>
#include <sstream>
#include <string>
#include <vector>

static std::array<std::pair<int, int>, 26 * 26 * 26> kGraph;

int GetValue(const std::string& node) {
  return (node[2] - 'A') + 26 * (node[1] - 'A') + 26 * 26 * (node[0] - 'A');
}

void UpdateGraph(const std::string& line, std::vector<int>& start_nodes) {
  int node_value = GetValue(line.substr(0, 3));
  kGraph[node_value] = {GetValue(line.substr(7, 3)),
                        GetValue(line.substr(12, 3))};
  if (line[2] == 'A') {
    start_nodes.push_back(node_value);
  }
}

int Navigate(const std::string& directions, int node) {
  int directions_index = 0;
  // The graph consistently reaches the same end node from the start node, and
  // d(start_node, end_node) = d(end_node, end_node). Therefore, we can stop the
  // exploration as soon as we reach the first end node.
  int steps = 0;
  do {
    if (directions[directions_index] == 'L') {
      node = kGraph[node].first;

    } else {
      node = kGraph[node].second;
    }
    ++directions_index;
    if (directions_index == directions.size()) directions_index = 0;
    ++steps;
  } while (node % 26 != 'Z' - 'A');
  return steps;
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  std::string directions;
  std::getline(iss, directions);
  std::string line;
  std::getline(iss, line);
  std::vector<int> start_nodes;
  while (std::getline(iss, line)) {
    UpdateGraph(line, start_nodes);
  }
  std::int64_t steps = 1;
  for (const auto& start_node : start_nodes) {
    steps = std::lcm(steps, Navigate(directions, start_node));
  }
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
