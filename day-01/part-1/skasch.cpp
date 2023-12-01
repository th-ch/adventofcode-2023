#include <algorithm>
#include <chrono>
#include <iostream>
#include <span>
#include <sstream>
#include <string>

bool IsInt(char c) { return c <= '9' && '0' <= c; }

int ToInt(char c) { return c - '0'; }

std::string Run(const std::string& input) {
  bool is_first_digit_found = false;
  int last_digit_found = 0;
  int result = 0;
  for (char c : input) {
    if (c == '\n') {
      is_first_digit_found = false;
      result += last_digit_found;
      last_digit_found = 0;
      continue;
    }
    if (!IsInt(c)) continue;
    last_digit_found = ToInt(c);
    if (!is_first_digit_found) {
      is_first_digit_found = true;
      result += 10 * last_digit_found;
    }
  }
  result += last_digit_found;
  return std::format("{}", result);
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
