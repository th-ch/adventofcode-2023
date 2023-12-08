#include <algorithm>
#include <chrono>
#include <cstdint>
#include <iostream>
#include <map>
#include <span>
#include <sstream>
#include <string>
#include <vector>

struct Mapping {
  std::int64_t dest_start;
  std::int64_t src_start;
  std::int64_t range;
};

struct ValueRange {
  std::int64_t start;
  std::int64_t range;
};

Mapping ParseMap(std::string& line) {
  int first_space = line.find(' ');
  int second_space = line.find(' ', first_space + 1);
  return {
      .dest_start = atoll(line.substr(0, first_space).c_str()),
      .src_start = atoll(
          line.substr(first_space + 1, second_space - first_space - 1).c_str()),
      .range = atoll(line.substr(second_space + 1).c_str())};
}

std::vector<ValueRange> ParseSeeds(std::string& line) {
  std::vector<ValueRange> seeds;
  for (int pos = 7; pos < line.size();) {
    int next_space = line.find(' ', pos);
    std::int64_t start = atoll(line.substr(pos, next_space - pos).c_str());
    pos = next_space + 1;
    next_space = line.find(' ', pos);
    if (next_space == -1) next_space = line.size();
    seeds.push_back(
        {.start = start,
         .range = atoll(line.substr(pos, next_space - pos).c_str())});
    pos = next_space + 1;
  }
  return seeds;
}

void UpdateValues(std::vector<ValueRange>& values,
                  const std::map<std::int64_t, Mapping>& mappings) {
  std::vector<ValueRange> new_values;
  for (const ValueRange& value : values) {
    std::int64_t cursor = value.start;
    while (cursor < value.start + value.range) {
      auto mapping = mappings.lower_bound(cursor);
      if (mapping == mappings.end()) {
        new_values.push_back(
            {.start = cursor, .range = value.start + value.range - cursor});
        break;
      }
      if (mapping->second.src_start <= cursor) {
        std::int64_t next_cursor =
            std::min(value.start + value.range,
                     mapping->second.src_start + mapping->second.range);
        new_values.push_back({.start = mapping->second.dest_start + cursor -
                                       mapping->second.src_start,
                              .range = next_cursor - cursor});
        std::swap(cursor, next_cursor);
      } else {
        std::int64_t next_cursor =
            std::min(value.start + value.range, mapping->second.src_start);
        new_values.push_back({.start = cursor, .range = next_cursor - cursor});
        std::swap(cursor, next_cursor);
      }
    }
  }
  std::swap(values, new_values);
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  std::string line;
  std::getline(iss, line);
  std::vector<ValueRange> values = ParseSeeds(line);
  std::getline(iss, line);
  std::getline(iss, line);
  std::map<std::int64_t, Mapping> mappings;
  for (std::string line; std::getline(iss, line);) {
    if (line == "") {
      std::getline(iss, line);
      UpdateValues(values, mappings);
      mappings.clear();
      continue;
    }
    Mapping mapping = ParseMap(line);
    mappings[mapping.src_start + mapping.range - 1] = mapping;
  }
  UpdateValues(values, mappings);
  return std::to_string(
      std::min_element(values.begin(), values.end(),
                       [](const ValueRange& left, const ValueRange& right) {
                         return left.start < right.start;
                       })
          ->start);
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
