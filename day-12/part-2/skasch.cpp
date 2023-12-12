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
static constexpr int kMaxLineSize = 21;
static constexpr int kMaxSizes = 7;
static std::array<std::int64_t, kNRrows * kRepetitions * kMaxLineSize *
                                    kRepetitions * kMaxLineSize * kRepetitions *
                                    kMaxSizes>
    kDp;

int ToIndex(int line_index, int left, int right, int index) {
  return line_index * kRepetitions * kMaxLineSize * kRepetitions *
             kMaxLineSize * kRepetitions * kMaxSizes +
         left * kRepetitions * kMaxLineSize * kRepetitions * kMaxSizes +
         right * kRepetitions * kMaxSizes + index;
}

std::int64_t Count(const std::string& line, const std::vector<int> sizes,
                   int line_index, int left, int right, int index) {
  std::int64_t* dp = &(kDp[ToIndex(line_index, left, right, index)]);
  if (*dp != 0) return *dp - 1;
  std::int64_t res = 0;
  if (left >= right) {
    res = index == sizes.size();
  } else {
    switch (line[left]) {
      case '.': {
        res = Count(line, sizes, line_index, left + 1, right, index);
        break;
      }
      case '?': {
        res = Count(line, sizes, line_index, left + 1, right, index);
      }
      case '#': {
        if (index >= sizes.size()) break;
        if (left + sizes[index] > right) break;
        bool has_hole = false;
        for (int pos = left + 1; pos < left + sizes[index]; ++pos) {
          if (line[pos] == '.') {
            has_hole = true;
            break;
          }
        }
        if (has_hole) break;
        int new_left = left + sizes[index];
        if (new_left >= right) {
          res += index + 1 == sizes.size();
          break;
        }
        if (line[new_left] == '#') break;
        res += Count(line, sizes, line_index, new_left + 1, right, index + 1);
        break;
      }
      default:
        throw std::invalid_argument("Invalid character.");
    }
  }
  *dp = res + 1;
  return res;
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
               0);
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
