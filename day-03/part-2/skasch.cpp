#include <chrono>
#include <cstddef>
#include <iostream>
#include <optional>
#include <sstream>
#include <string>

bool IsDigit(char c) { return '0' <= c && c <= '9'; }

struct ValueAndNextPos {
  int value;
  int pos;
};

std::optional<ValueAndNextPos> ParseNumberAt(const std::string& line, int pos) {
  if (!IsDigit(line[pos])) return std::nullopt;
  int left = pos;
  while (left > 0 && IsDigit(line[left - 1])) --left;
  ++pos;
  while (pos < line.size() && IsDigit(line[pos])) ++pos;
  return {{.value = atoi(line.substr(left, pos - left).c_str()), .pos = pos}};
}

int ParseLine(const std::string& line, const std::string& previous_line,
              const std::string& next_line) {
  int result = 0;
  for (int pos = 0; pos < line.size(); ++pos) {
    if (line[pos] != '*') continue;
    int value = 1;
    int numbers = 0;
    int num_pos = pos - 1;
    while (num_pos <= pos + 1) {
      std::optional<ValueAndNextPos> value_and_next_pos =
          ParseNumberAt(previous_line, num_pos);
      if (!value_and_next_pos.has_value()) {
        ++num_pos;
        continue;
      }
      value *= value_and_next_pos->value;
      num_pos = value_and_next_pos->pos;
      ++numbers;
    }
    num_pos = pos - 1;
    while (num_pos <= pos + 1) {
      std::optional<ValueAndNextPos> value_and_next_pos =
          ParseNumberAt(next_line, num_pos);
      if (!value_and_next_pos.has_value()) {
        ++num_pos;
        continue;
      }
      value *= value_and_next_pos->value;
      num_pos = value_and_next_pos->pos;
      ++numbers;
    }
    if (std::optional<ValueAndNextPos> value_and_next_pos =
            ParseNumberAt(line, pos - 1);
        value_and_next_pos.has_value()) {
      value *= value_and_next_pos->value;
      ++numbers;
    }
    if (std::optional<ValueAndNextPos> value_and_next_pos =
            ParseNumberAt(line, pos + 1);
        value_and_next_pos.has_value()) {
      value *= value_and_next_pos->value;
      pos = value_and_next_pos->pos - 1;
      ++numbers;
    }
    if (numbers >= 2) {
      result += value;
    }
  }
  return result;
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  int result = 0;
  std::string previous_line;
  std::getline(iss, previous_line);
  std::string line;
  std::getline(iss, line);
  std::string next_line;
  result += ParseLine(previous_line, next_line, line);
  for (; std::getline(iss, next_line);) {
    result += ParseLine(line, previous_line, next_line);
    std::swap(previous_line, line);
    std::swap(line, next_line);
  }
  next_line.clear();
  result += ParseLine(line, previous_line, next_line);
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
