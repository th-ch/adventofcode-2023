#include <chrono>
#include <cmath>
#include <cstdlib>
#include <iostream>
#include <span>
#include <sstream>
#include <string>

static constexpr double kEpsilon = 0.0001;

bool IsDigit(char c) { return '0' <= c && c <= '9'; }

std::int64_t ParseValue(const std::string& line) {
  std::string number;
  for (char c : line.substr(9)) {
    if (!IsDigit(c)) continue;
    number.push_back(c);
  }
  return atoll(number.c_str());
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
  std::int64_t total_time = ParseValue(line);
  std::getline(iss, line);
  std::int64_t distance = ParseValue(line);
  return std::to_string(CountDistinctWays(total_time, distance));
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
