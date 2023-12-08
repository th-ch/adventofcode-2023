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

Mapping ParseMap(std::string& line) {
  int first_space = line.find(' ');
  int second_space = line.find(' ', first_space + 1);
  return {
      .dest_start = atoll(line.substr(0, first_space).c_str()),
      .src_start = atoll(
          line.substr(first_space + 1, second_space - first_space - 1).c_str()),
      .range = atoll(line.substr(second_space + 1).c_str())};
}

std::vector<std::int64_t> ParseSeeds(std::string& line) {
  std::vector<std::int64_t> seeds;
  for (int pos = 7; pos < line.size();) {
    int next_space = line.find(' ', pos);
    if (next_space == -1) next_space = line.size();
    seeds.push_back(atoll(line.substr(pos, next_space - pos).c_str()));
    pos = next_space + 1;
  }
  return seeds;
}

void UpdateValues(std::vector<std::int64_t>& values,
                  const std::map<std::int64_t, Mapping>& mappings) {
  std::vector<std::int64_t> new_values;
  for (std::int64_t value : values) {
    auto mapping = mappings.lower_bound(value);
    if (mapping == mappings.end()) {
      new_values.push_back(value);
      continue;
    }
    if (mapping->second.src_start <= value) {
      new_values.push_back(mapping->second.dest_start + value -
                           mapping->second.src_start);
    } else {
      new_values.push_back(value);
    }
  }
  std::swap(values, new_values);
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  std::string line;
  std::getline(iss, line);
  std::vector<std::int64_t> values = ParseSeeds(line);
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
  return std::to_string(*std::min_element(values.begin(), values.end()));
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
