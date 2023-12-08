#include <chrono>
#include <cmath>
#include <iostream>
#include <span>
#include <sstream>
#include <string>
#include <vector>

static constexpr double kEpsilon = 0.0001;

std::vector<int> ParseValues(const std::string& line) {
  std::vector<int> values;
  for (int pos = 9; pos < line.size(); pos += 7) {
    values.push_back(atoi(line.substr(pos, 7).c_str()));
  }
  return values;
}

int CountDistinctWays(double time, double distance) {
  double delta = time * time - 4.0 * distance;
  if (delta < 0) return 0;
  double sqrt_delta = std::sqrt(delta);
  return std::floor((time + sqrt_delta) / 2.0 - kEpsilon) -
         std::ceil((time - sqrt_delta) / 2.0 + kEpsilon) + 1;
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  std::string line;
  std::getline(iss, line);
  std::vector<int> times = ParseValues(line);
  std::getline(iss, line);
  std::vector<int> distances = ParseValues(line);
  int result = 1;
  for (auto idx = 0; idx < times.size(); ++idx) {
    result *= CountDistinctWays(times[idx], distances[idx]);
  }
  return std::to_string(result);
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
