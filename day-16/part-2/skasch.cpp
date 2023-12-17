#include <array>
#include <chrono>
#include <cstddef>
#include <iostream>
#include <span>
#include <stdexcept>
#include <string>
#include <vector>

static constexpr int kSize = 110;
static constexpr int kInputSize = kSize * (kSize + 1) - 1;
static std::array<bool, 5 * kInputSize * kSize * 4> kEnergized;
static constexpr int kRight = 1;
static constexpr int kLeft = -1;
static constexpr int kUp = -kSize - 1;
static constexpr int kDown = kSize + 1;

int ToIndex(int direction) {
  switch (direction) {
    case kRight:
      return 1;
    case kLeft:
      return 2;
    case kUp:
      return 3;
    case kDown:
      return 4;
    default:
      throw std::invalid_argument("ToIndex: invalid direction.");
  }
}

int CountEnergized(const std::string& input, int start_pos, int start_dir,
                   int call) {
  std::vector<std::pair<int, int>> positions = {{start_pos, start_dir}};
  int count_energized = 0;
  while (!positions.empty()) {
    std::vector<std::pair<int, int>> next_positions;
    for (const auto& [pos, dir] : positions) {
      int next_pos = pos + dir;
      if (next_pos < 0 || next_pos >= kInputSize || input[next_pos] == '\n')
        continue;
      if (kEnergized[5 * kInputSize * call + ToIndex(dir) * kInputSize +
                     next_pos])
        continue;
      kEnergized[5 * kInputSize * call + ToIndex(dir) * kInputSize + next_pos] =
          true;
      if (!kEnergized[5 * kInputSize * call + next_pos]) {
        kEnergized[5 * kInputSize * call + next_pos] = true;
        ++count_energized;
      }
      switch (input[next_pos]) {
        case '.': {
          next_positions.push_back({next_pos, dir});
          break;
        }
        case '\\': {
          switch (dir) {
            case kRight: {
              next_positions.push_back({next_pos, kDown});
              break;
            }
            case kLeft: {
              next_positions.push_back({next_pos, kUp});
              break;
            }
            case kUp: {
              next_positions.push_back({next_pos, kLeft});
              break;
            }
            case kDown: {
              next_positions.push_back({next_pos, kRight});
              break;
            }
            default:
              throw std::invalid_argument("Invalid direction.");
          }
          break;
        }
        case '/': {
          switch (dir) {
            case kRight: {
              next_positions.push_back({next_pos, kUp});
              break;
            }
            case kLeft: {
              next_positions.push_back({next_pos, kDown});
              break;
            }
            case kUp: {
              next_positions.push_back({next_pos, kRight});
              break;
            }
            case kDown: {
              next_positions.push_back({next_pos, kLeft});
              break;
            }
            default:
              throw std::invalid_argument("Invalid direction.");
          }
          break;
        }
        case '-': {
          switch (dir) {
            case kRight:
            case kLeft: {
              next_positions.push_back({next_pos, dir});
              break;
            }
            case kUp:
            case kDown: {
              next_positions.push_back({next_pos, kRight});
              next_positions.push_back({next_pos, kLeft});
              break;
            }
            default:
              throw std::invalid_argument("Invalid direction.");
          }
          break;
        }
        case '|': {
          switch (dir) {
            case kRight:
            case kLeft: {
              next_positions.push_back({next_pos, kUp});
              next_positions.push_back({next_pos, kDown});
              break;
            }
            case kUp:
            case kDown: {
              next_positions.push_back({next_pos, dir});
              break;
            }
            default:
              throw std::invalid_argument("Invalid direction.");
          }
          break;
        }
        default:
          throw std::invalid_argument("Invalid character.");
      }
    }
    std::swap(positions, next_positions);
  }
  return count_energized;
}

std::string Run(const std::string& input) {
  // Your code goes here
  int res = 0;
  int call = 0;
  for (int row = 0; row < kSize; ++row) {
    res = std::max(
        res, CountEnergized(input, kLeft + row * (kSize + 1), kRight, call));
    ++call;
    res = std::max(res,
                   CountEnergized(input, kRight + kSize - 1 + row * (kSize + 1),
                                  kLeft, call));
    ++call;
  }
  for (int col = 0; col < kSize; ++col) {
    res = std::max(res, CountEnergized(input, kUp + col, kDown, call));
    ++call;
    res = std::max(
        res, CountEnergized(input, (kSize - 1) * (kSize + 1) + kDown + col, kUp,
                            call));
    ++call;
  }
  return std::to_string(res);
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
