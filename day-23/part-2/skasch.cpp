#include <chrono>
#include <cstddef>
#include <deque>
#include <iostream>
#include <optional>
#include <span>
#include <string>
#include <tuple>
#include <unordered_map>
#include <vector>

static constexpr int kSize = 141;
static constexpr int kStart = 1;
static constexpr int kEnd = (kSize + 1) * (kSize - 1) + kSize - 2;
static constexpr int kLeft = -1;
static constexpr int kRight = 1;
static constexpr int kUp = -(kSize + 1);
static constexpr int kDown = kSize + 1;
static const std::vector<int> kDirections = {kLeft, kUp, kRight, kDown};
static const std::unordered_map<int, int> kDirectionsIndex = {
    {kLeft, 0}, {kUp, 1}, {kRight, 2}, {kDown, 3}};
static constexpr int kNNodes = 64;
static std::array<std::unordered_map<int, int>, kNNodes> kGraph;
static std::array<int, kNNodes> kPos = {kStart, kEnd};
static std::unordered_map<int, int> kIndex = {{kStart, 0}, {kEnd, 1}};
static std::array<std::array<bool, 4>, kNNodes> kVisitedDirections;
static std::array<bool, kNNodes> kVisited;

static int kCountNodes = 2;

bool IsValid(int index, const std::string& input) {
  return index >= kStart && index <= kEnd && input[index] != '#' &&
         input[index] != '\n';
}

std::optional<std::tuple<int, int, int>> Explore(const std::string& input,
                                                 int pos, int direction) {
  int distance = 1;
  while (!kIndex.contains(pos)) {
    bool found = false;
    for (int dir : kDirections) {
      if (dir == -direction) continue;
      if (input[pos + dir] == '#') continue;
      pos += dir;
      direction = dir;
      ++distance;
      found = true;
      break;
    }
    if (!found) {
      return std::nullopt;
    }
  }
  return std::make_tuple(pos, direction, distance);
}

void FindVertexes(const std::string& input) {
  bool left_empty = false;
  for (int pos = kSize + 2; pos < kEnd - kSize; ++pos) {
    if (input[pos] == '\n') {
      ++pos;
    }
    if (input[pos] == '#') {
      left_empty = false;
      continue;
    }
    int count = left_empty + (input[pos + kRight] != '#') +
                (input[pos + kUp] != '#') + (input[pos + kDown] != '#');
    if (count >= 3) {
      kPos[kCountNodes] = pos;
      kIndex[pos] = kCountNodes;
      ++kCountNodes;
    }
    left_empty = true;
  }
}

void BuildGraph(const std::string& input) {
  FindVertexes(input);
  for (int index = 2; index < kCountNodes; ++index) {
    for (int direction_index = 0; direction_index < 4; ++direction_index) {
      int direction = kDirections[direction_index];
      if (kVisitedDirections[index][direction_index]) continue;
      int pos = kPos[index];
      if (input[pos + direction] == '#') {
        kVisitedDirections[index][direction_index] = true;
        continue;
      }
      const auto result = Explore(input, pos + direction, direction);
      if (result.has_value()) {
        const auto& [end_pos, end_direction, distance] = *result;
        int end_index = kIndex[end_pos];
        kGraph[index][end_index] = distance;
        kGraph[end_index][index] = distance;
        kVisitedDirections[end_index]
                          [(kDirectionsIndex.at(end_direction) + 2) % 4] = true;
      }
      kVisitedDirections[index][direction_index] = true;
    }
  }
}

int FindLongestPath(const std::string& input) {
  std::vector<int> path;
  std::size_t total_distance = 0;
  std::deque<int> actions = {0};
  std::size_t longest = 0;
  while (!actions.empty()) {
    int action = actions.front();
    actions.pop_front();
    if (action == -1) {
      int prev_pos = path.back();
      kVisited[prev_pos] = false;
      path.pop_back();
      total_distance -= kGraph[prev_pos][path.back()];
      continue;
    }
    if (!path.empty()) {
      total_distance += kGraph[path.back()][action];
    }
    kVisited[action] = true;
    path.push_back(action);
    if (action == 1) {
      longest = std::max(longest, total_distance);
      continue;
    }
    for (const auto& [next_pos, _] : kGraph[action]) {
      if (kVisited[next_pos]) continue;
      actions.push_front(-1);
      actions.push_front(next_pos);
    }
  }
  return longest;
}

std::string Run(const std::string& input) {
  // Your code goes here
  BuildGraph(input);
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
