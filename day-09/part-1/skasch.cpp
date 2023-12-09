#include <chrono>
#include <deque>
#include <iostream>
#include <span>
#include <sstream>
#include <string>
#include <vector>

std::deque<int> ParseLine(const std::string& line) {
  std::deque<int> numbers;
  int pos = 0;
  do {
    int next_pos = line.find(' ', pos);
    numbers.push_back(atoi(line.substr(pos, next_pos - pos).c_str()));
    pos = next_pos + 1;
  } while (pos > 0);
  return numbers;
}

int FindNext(const std::deque<int>& numbers) {
  std::vector<std::vector<int>> diffs = {
      {numbers.back() - numbers.at(numbers.size() - 2)}};
  while (true) {
    diffs.front().push_back(
        numbers.at(numbers.size() - diffs.front().size() - 1) -
        numbers.at(numbers.size() - diffs.front().size() - 2));
    for (int idx = 1; idx < diffs.size(); ++idx) {
      diffs.at(idx).push_back(
          diffs.at(idx - 1).at(diffs.at(idx - 1).size() - 2) -
          diffs.at(idx - 1).back());
    }
    if (diffs.size() == numbers.size() - 2) {
      diffs.pop_back();
      break;
    }
    if (diffs.size() >= 2 && diffs.at(diffs.size() - 2).front() == 0 &&
        diffs.at(diffs.size() - 2).at(1) == 0 &&
        diffs.at(diffs.size() - 2).back() == 0) {
      diffs.pop_back();
      diffs.pop_back();
      break;
    }
    diffs.push_back({diffs.back().front() - diffs.back().back()});
  }
  int result = numbers.back();
  for (const std::vector<int>& diff : diffs) {
    result += diff.front();
  }
  return result;
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  int result = 0;
  for (std::string line; std::getline(iss, line);) {
    result += FindNext(ParseLine(line));
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
