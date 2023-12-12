#include <array>
#include <chrono>
#include <cstdint>
#include <iostream>
#include <span>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

static constexpr int kRepetitions = 5;
static constexpr int kNRrows = 1000;
static constexpr int kMaxLineSize = 25;
static constexpr int kMaxSizes = 10;
static std::array<std::int64_t, kNRrows * kRepetitions * kMaxLineSize *
                                    kRepetitions * kMaxLineSize * kRepetitions *
                                    kMaxSizes * 2>
    kDp;

int ToIndex(int line_index, int left, int right, int index,
            bool prev_is_spring) {
  int factor = kRepetitions * kMaxLineSize * kRepetitions * kMaxLineSize *
               kRepetitions * kMaxSizes * 2;
  int result = line_index * factor;
  factor /= kRepetitions * kMaxLineSize;
  result += left * factor;
  factor /= kRepetitions * kMaxLineSize;
  result += right * factor;
  factor /= kRepetitions * kMaxSizes;
  result += index * factor;
  result += prev_is_spring;
  return result;
}

std::int64_t Count(const std::string& line, const std::vector<int> sizes,
                   int line_index, int left, int right, int index,
                   bool prev_is_spring) {
  std::int64_t* dp =
      &(kDp[ToIndex(line_index, left, right, index, prev_is_spring)]);
  if (*dp != 0) return *dp - 1;
  if (left >= right) {
    std::int64_t res = index == sizes.size();
    *dp = res + 1;
    return res;
  }
  switch (line[left]) {
    case '.': {
      std::int64_t res =
          Count(line, sizes, line_index, left + 1, right, index, false);
      *dp = res + 1;
      return res;
    }
    case '#': {
      if (index >= sizes.size()) return 0;
      if (prev_is_spring) return 0;
      if (left + sizes[index] > right) return 0;
      for (int pos = left + 1; pos < left + sizes[index]; ++pos) {
        if (line[pos] == '.') return 0;
      }
      std::int64_t res = Count(line, sizes, line_index, left + sizes[index],
                               right, index + 1, true);
      *dp = res + 1;
      return res;
    }
    case '?': {
      std::int64_t res =
          Count(line, sizes, line_index, left + 1, right, index, false);
      if (index >= sizes.size()) return res;
      if (prev_is_spring) return res;
      if (left + sizes[index] > right) return res;
      for (int pos = left + 1; pos < left + sizes[index]; ++pos) {
        if (line[pos] == '.') return res;
      }
      res += Count(line, sizes, line_index, left + sizes[index], right,
                   index + 1, true);
      *dp = res + 1;
      return res;
    }
    default:
      throw std::invalid_argument("Invalid character.");
  }
}

std::int64_t ParseLine(int line_index, const std::string& line) {
  int space = line.find(' ');
  std::string multiline(line.substr(0, space));
  std::vector<int> sizes;
  for (int pos = space + 1; pos < line.size();) {
    int comma = line.find(',', pos);
    if (comma < 0) comma = line.size();
    sizes.push_back(atoi(line.substr(pos, comma - pos).c_str()));
    pos = comma + 1;
  }
  int sizes_size = sizes.size();
  for (int rep = 1; rep < kRepetitions; ++rep) {
    multiline.push_back('?');
    for (int pos = 0; pos < space; ++pos) multiline.push_back(line[pos]);
    for (int idx = 0; idx < sizes_size; ++idx) sizes.push_back(sizes[idx]);
  }
  return Count(multiline, sizes, line_index, 0, kRepetitions * (space + 1) - 1,
               0, false);
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  std::int64_t result = 0;
  int line_index = 0;
  for (std::string line; std::getline(iss, line); ++line_index) {
    result += ParseLine(line_index, line);
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
