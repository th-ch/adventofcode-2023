#include <chrono>
#include <cstdlib>
#include <functional>
#include <iostream>
#include <queue>
#include <span>
#include <stdexcept>
#include <string>

static constexpr int kSize = 141;
static constexpr int kInputSize = kSize * (kSize + 1) - 1;
static std::vector<bool> kVisited(kInputSize * 4 * 10, false);
static constexpr int kRight = 1;
static constexpr int kLeft = -1;
static constexpr int kDown = kSize + 1;
static constexpr int kUp = -kSize - 1;

int DirToEnum(int direction) {
  switch (direction) {
    case kRight:
      return 0;
    case kLeft:
      return 1;
    case kDown:
      return 2;
    case kUp:
      return 3;
    default:
      throw std::invalid_argument("Invalid direction.");
  }
}

int EnumToDir(int index) {
  switch (index) {
    case 0:
      return kRight;
    case 1:
      return kLeft;
    case 2:
      return kDown;
    case 3:
      return kUp;
    default:
      throw std::invalid_argument("Invalid direction index.");
  }
}

int StateToIndex(int pos, int direction, int steps) {
  return kInputSize * 4 * (steps - 1) + kInputSize * DirToEnum(direction) + pos;
}

int IndexToPos(int index) { return index % kInputSize; }
int IndexToDir(int index) { return EnumToDir((index / kInputSize) % 4); }
int IndexToSteps(int index) { return index / (4 * kInputSize) + 1; }

int HeatLoss(char c) { return c - '0'; }

bool IsValid(int pos) {
  return pos >= 0 && pos < kInputSize && pos % (kSize + 1) != kSize;
}

std::vector<int> ValidDirections(int direction, int steps) {
  if (steps <= 3) {
    return {direction};
  }
  if (steps < 10) {
    switch (direction) {
      case kRight:
        return {kRight, kUp, kDown};
      case kLeft:
        return {kLeft, kUp, kDown};
      case kUp:
        return {kUp, kLeft, kRight};
      case kDown:
        return {kDown, kLeft, kRight};
      default:
        throw std::invalid_argument("Invalid direction.");
    }
  }
  switch (direction) {
    case kRight:
    case kLeft:
      return {kUp, kDown};
    case kUp:
    case kDown:
      return {kRight, kLeft};
    default:
      throw std::invalid_argument("Invalid direction.");
  }
}

using PQ =
    std::priority_queue<std::pair<int, int>, std::vector<std::pair<int, int>>,
                        std::greater<std::pair<int, int>>>;

int FindShortest(const std::string& input) {
  PQ pq;
  pq.push({HeatLoss(input[1]), StateToIndex(1, kRight, 1)});
  pq.push({HeatLoss(input[kSize + 1]), StateToIndex(kSize + 1, kDown, 1)});
  while (!pq.empty()) {
    auto [heat, index] = pq.top();
    pq.pop();
    int pos = IndexToPos(index);
    if (kVisited[index]) continue;
    kVisited[index] = true;
    int direction = IndexToDir(index);
    int steps = IndexToSteps(index);
    for (int next_dir : ValidDirections(direction, steps)) {
      int next_pos = pos + next_dir;
      if (!IsValid(next_pos)) continue;
      int next_heat = heat + HeatLoss(input[next_pos]);
      if (next_pos == kInputSize - 1) {
        return next_heat;
      }
      int next_steps = (next_dir == direction) ? steps + 1 : 1;
      pq.push({next_heat, StateToIndex(next_pos, next_dir, next_steps)});
    }
  }
  throw std::invalid_argument("No path found.");
}

std::string Run(const std::string& input) {
  // Your code goes here
  return std::to_string(FindShortest(input));
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
