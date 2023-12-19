#include <chrono>
#include <iostream>
#include <span>
#include <sstream>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <utility>

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

struct Shape {
  int x;
  int m;
  int a;
  int s;

  int Get(char c) const {
    switch (c) {
      case 'x':
        return x;
      case 'm':
        return m;
      case 'a':
        return a;
      case 's':
        return s;
      default:
        throw std::invalid_argument("Invalid character for shape.");
    }
  }
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

Shape ParseShape(const std::string& line) {
  int left = line.find(',');
  int x = std::atoi(line.substr(3, left - 3).c_str());
  int right = line.find(',', left + 1);
  int m = std::atoi(line.substr(left + 3, right - left - 3).c_str());
  std::swap(left, right);
  right = line.find(',', left + 1);
  int a = std::atoi(line.substr(left + 3, right - left - 3).c_str());
  std::swap(left, right);
  right = line.find('}', left + 1);
  int s = std::atoi(line.substr(left + 3, right - left - 3).c_str());
  return {x, m, a, s};
}

bool IsAccepted(const std::string& input, const Workflows& workflows,
                const Shape& shape, const Workflow& workflow) {
  for (const Condition& condition : workflow.conditions) {
    bool result;
    switch (input[condition.start + 1]) {
      case '>': {
        result = shape.Get(input[condition.start]) > condition.threshold;
        break;
      }
      case '<': {
        result = shape.Get(input[condition.start]) < condition.threshold;
        break;
      }
      default: {
        throw std::invalid_argument("Invalid condition operator.");
      }
    }
    if (!result) continue;
    if (condition.target_size == 1) {
      switch (input[condition.target_start]) {
        case 'A':
          return true;
        case 'R':
          return false;
      }
    }
    return IsAccepted(input, workflows, shape,
                      workflows.at(input.substr(condition.target_start,
                                                condition.target_size)));
  }
  if (workflow.else_size == 1) {
    switch (input[workflow.else_start]) {
      case 'A':
        return true;
      case 'R':
        return false;
    }
  }
  return IsAccepted(
      input, workflows, shape,
      workflows.at(input.substr(workflow.else_start, workflow.else_size)));
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
  int result = 0;
  for (std::string line; std::getline(iss, line);) {
    Shape shape = ParseShape(line);
    bool is_accepted = IsAccepted(input, workflows, shape, workflows["in"]);
    if (is_accepted) result += shape.x + shape.m + shape.a + shape.s;
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
