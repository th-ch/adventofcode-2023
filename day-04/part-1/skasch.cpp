#include <chrono>
#include <cstddef>
#include <iostream>
#include <set>
#include <span>
#include <sstream>
#include <string>

int GetCardScore(const std::string& line) {
  std::set<int> winning_numbers = {};
  for (std::size_t pos = 10; pos < 39; pos += 3) {
    winning_numbers.insert(atoi(line.substr(pos, 2).c_str()));
  }
  int score = 0;
  for (std::size_t pos = 42; pos < line.size(); pos += 3) {
    if (winning_numbers.contains(atoi(line.substr(pos, 2).c_str()))) {
      if (score == 0) {
        score = 1;
      } else {
        score *= 2;
      }
    }
  }
  return score;
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  int result = 0;
  for (std::string line; std::getline(iss, line);) {
    result += GetCardScore(line);
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
