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
static constexpr int kMinSteps = 4;
static constexpr int kMaxSteps = 10;
static std::vector<bool> kVisited(kInputSize * 4, false);
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

int StateToIndex(int pos, int direction) {
  return kInputSize * DirToEnum(direction) + pos;
}

int IndexToPos(int index) { return index % kInputSize; }
int IndexToDir(int index) { return EnumToDir(index / kInputSize); }

int HeatLoss(char c) { return c - '0'; }

bool IsValid(int pos) {
  return pos >= 0 && pos < kInputSize && pos % (kSize + 1) != kSize;
}

using PQ =
    std::priority_queue<std::pair<int, int>, std::vector<std::pair<int, int>>,
                        std::greater<std::pair<int, int>>>;

int FindShortest(const std::string& input) {
  PQ pq;
  pq.push({0, StateToIndex(0, kRight)});
  pq.push({0, StateToIndex(0, kDown)});
  while (!pq.empty()) {
    auto [heat, index] = pq.top();
    pq.pop();
    int pos = IndexToPos(index);
    if (pos == kInputSize - 1) {
      return heat;
    }
    if (kVisited[index]) continue;
    kVisited[index] = true;
    int direction = IndexToDir(index);
    int next_pos = pos;
    int next_heat = heat;
    for (int steps = 1; steps <= kMaxSteps; ++steps) {
      next_pos += direction;
      if (!IsValid(next_pos)) break;
      next_heat += HeatLoss(input[next_pos]);
      if (steps < kMinSteps) continue;
      switch (direction) {
        case kLeft:
        case kRight: {
          pq.push({next_heat, StateToIndex(next_pos, kUp)});
          pq.push({next_heat, StateToIndex(next_pos, kDown)});
          break;
        }
        case kUp:
        case kDown: {
          pq.push({next_heat, StateToIndex(next_pos, kRight)});
          pq.push({next_heat, StateToIndex(next_pos, kLeft)});
          break;
        }
        default:
          throw std::invalid_argument("Invalid direction");
      }
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
