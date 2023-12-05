#include <chrono>
#include <cstddef>
#include <deque>
#include <iostream>
#include <span>
#include <sstream>
#include <string>

bool IsDigit(char c) { return '0' <= c && c <= '9'; }

struct Number {
  int left;
  int right;
  int value;
};

int ParseLine(const std::string& line, std::deque<int>& previous_characters,
              std::deque<Number>& previous_numbers,
              std::deque<int>& current_characters,
              std::deque<Number>& current_numbers) {
  bool previous_is_character = false;
  int result = 0;
  for (int pos = 0; pos < line.size(); ++pos) {
    if (line[pos] == '.') {
      previous_is_character = false;
      continue;
    }
    if (IsDigit(line[pos])) {
      int left = pos;
      while (IsDigit(line[pos + 1])) {
        ++pos;
      }
      int value = atoi(line.substr(left, pos - left + 1).c_str());
      if (previous_is_character ||
          (pos + 1 < line.size() && line[pos + 1] != '.' &&
           !IsDigit(line[pos + 1]))) {
        result += value;
        previous_is_character = false;
        continue;
      }
      previous_is_character = false;
      while (!previous_characters.empty() &&
             previous_characters.front() < left - 1) {
        previous_characters.pop_front();
      }
      if (!previous_characters.empty() &&
          previous_characters.front() <= pos + 1) {
        result += value;
        continue;
      }
      current_numbers.push_back(
          Number{.left = left, .right = pos, .value = value});
      continue;
    }
    previous_is_character = true;
    current_characters.push_back(pos);
    while (!previous_numbers.empty() &&
           previous_numbers.front().right + 1 < pos) {
      previous_numbers.pop_front();
    }
    while (!previous_numbers.empty() &&
           previous_numbers.front().left <= pos + 1) {
      result += previous_numbers.front().value;
      previous_numbers.pop_front();
    }
  }
  return result;
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  std::deque<int> previous_characters;
  std::deque<int> current_characters;
  std::deque<Number> previous_numbers;
  std::deque<Number> current_numbers;
  int result = 0;
  for (std::string line; std::getline(iss, line);) {
    result += ParseLine(line, previous_characters, previous_numbers,
                        current_characters, current_numbers);
    std::swap(previous_characters, current_characters);
    current_characters.clear();
    std::swap(previous_numbers, current_numbers);
    current_numbers.clear();
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
