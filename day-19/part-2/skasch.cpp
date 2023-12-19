#include <algorithm>
#include <chrono>
#include <cstdint>
#include <iostream>
#include <span>
#include <sstream>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

struct Condition {
  int start;
  int threshold;
  int target_start;
  int target_size;
};

struct Workflow {
  std::vector<Condition> conditions;
  int else_start;
  int else_size;
};

using Workflows = std::unordered_map<std::string, Workflow>;

void ParseWorkflow(const std::string& line, int start, Workflows& workflows) {
  int left_brace = line.find('{');
  Workflow& workflow = workflows[line.substr(0, left_brace)];
  int pos = left_brace + 1;
  for (; pos < line.size(); ++pos) {
    int sep = line.find(':', pos);
    if (sep == -1) break;
    int threshold = std::atoi(line.substr(pos + 2, sep - pos - 2).c_str());
    int next_pos = line.find(',', sep);
    workflow.conditions.push_back(
        {start + pos, threshold, start + sep + 1, next_pos - sep - 1});
    std::swap(pos, next_pos);
  }
  workflow.else_start = start + pos;
  workflow.else_size = line.size() - pos - 1;
}

static constexpr int kMaxValue = 4000;

struct Bounds {
  std::int64_t left_x = 1;
  std::int64_t right_x = kMaxValue;
  std::int64_t left_m = 1;
  std::int64_t right_m = kMaxValue;
  std::int64_t left_a = 1;
  std::int64_t right_a = kMaxValue;
  std::int64_t left_s = 1;
  std::int64_t right_s = kMaxValue;

  std::int64_t& Left(char c) {
    switch (c) {
      case 'x':
        return left_x;
      case 'm':
        return left_m;
      case 'a':
        return left_a;
      case 's':
        return left_s;
      default:
        throw std::invalid_argument("Invalid character for Left.");
    }
  }
  std::int64_t& Right(char c) {
    switch (c) {
      case 'x':
        return right_x;
      case 'm':
        return right_m;
      case 'a':
        return right_a;
      case 's':
        return right_s;
      default:
        throw std::invalid_argument("Invalid character for Left.");
    }
  }

  std::int64_t Size() const {
    return (right_x - left_x + 1) * (right_m - left_m + 1) *
           (right_a - left_a + 1) * (right_s - left_s + 1);
  }
};

std::int64_t CountAccepted(const std::string& input, const Workflows& workflows,
                           const Workflow& workflow, Bounds bounds) {
  std::int64_t result = 0;
  int tmp;
  for (const Condition& condition : workflow.conditions) {
    switch (input[condition.start + 1]) {
      case '>': {
        tmp = bounds.Left(input[condition.start]);
        bounds.Left(input[condition.start]) = condition.threshold + 1;
        break;
      }
      case '<': {
        tmp = bounds.Right(input[condition.start]);
        bounds.Right(input[condition.start]) = condition.threshold - 1;
        break;
      }
      default: {
        throw std::invalid_argument("Invalid condition operator.");
      }
    }
    if (condition.target_size == 1) {
      switch (input[condition.target_start]) {
        case 'A': {
          result += bounds.Size();
          break;
        }
        case 'R': {
          break;
        }
        default: {
          throw std::invalid_argument("Unexpected workflow name of size 1.");
        }
      }
    } else {
      result += CountAccepted(input, workflows,
                              workflows.at(input.substr(condition.target_start,
                                                        condition.target_size)),
                              bounds);
    }
    switch (input[condition.start + 1]) {
      case '>': {
        bounds.Left(input[condition.start]) = tmp;
        bounds.Right(input[condition.start]) = condition.threshold;
        break;
      }
      case '<': {
        bounds.Right(input[condition.start]) = tmp;
        bounds.Left(input[condition.start]) = condition.threshold;
        break;
      }
      default: {
        throw std::invalid_argument("Invalid condition operator.");
      }
    }
  }
  if (workflow.else_size == 1) {
    switch (input[workflow.else_start]) {
      case 'A': {
        result += bounds.Size();
        break;
      }
      case 'R': {
        break;
      }
      default: {
        throw std::invalid_argument("Unexpected workflow name of size 1.");
      }
    }
  } else {
    result += CountAccepted(
        input, workflows,
        workflows.at(input.substr(workflow.else_start, workflow.else_size)),
        bounds);
  }
  return result;
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  Workflows workflows;
  int start = 0;
  for (std::string line; std::getline(iss, line);) {
    if (line.size() == 0) {
      break;
    }
    ParseWorkflow(line, start, workflows);
    start += line.size() + 1;
  }
  Bounds test(1, 2, 3, 4, 5, 6, 7, 8);
  return std::to_string(
      CountAccepted(input, workflows, workflows.at("in"), Bounds()));
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
