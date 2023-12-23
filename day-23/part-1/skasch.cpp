#include <chrono>
#include <cstddef>
#include <deque>
#include <iostream>
#include <span>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

static constexpr int kSize = 141;
static constexpr int kStart = 1;
static constexpr int kEnd = (kSize + 1) * (kSize - 1) + kSize - 2;
static constexpr int kLeft = -1;
static constexpr int kRight = 1;
static constexpr int kUp = -(kSize + 1);
static constexpr int kDown = kSize + 1;
static const std::unordered_map<char, std::vector<int>> kDirections = {
    {'.', {kLeft, kRight, kUp, kDown}},
    {'>', {kRight}},
    {'<', {kLeft}},
    {'^', {kUp}},
    {'v', {kDown}}};

bool IsValid(int index, const std::string& input) {
  return index >= kStart && index <= kEnd && input[index] != '#' &&
         input[index] != '\n';
}

int FindLongestPath(const std::string& input) {
  std::vector<int> path;
  std::unordered_set<int> visited;
  std::deque<int> actions = {kStart};
  std::size_t longest = 0;
  while (!actions.empty()) {
    int action = actions.front();
    actions.pop_front();
    if (action == -1) {
      visited.erase(path.back());
      path.pop_back();
      continue;
    }
    visited.insert(action);
    path.push_back(action);
    if (action == kEnd) {
      longest = std::max(longest, path.size() - 1);
      continue;
    }
    for (int direction : kDirections.at(input[action])) {
      int next_pos = action + direction;
      if (!IsValid(next_pos, input)) continue;
      if (visited.contains(next_pos)) continue;
      actions.push_front(-1);
      actions.push_front(next_pos);
    }
  }
  return longest;
}

std::string Run(const std::string& input) {
  // Your code goes here
  return std::to_string(FindLongestPath(input));
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
