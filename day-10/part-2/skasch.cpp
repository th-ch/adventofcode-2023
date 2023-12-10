#include <cassert>
#include <chrono>
#include <iostream>
#include <optional>
#include <span>
#include <stdexcept>
#include <string>
#include <unordered_set>
#include <utility>

static constexpr int kRowSize = 140;
static int kInputSize;

std::optional<int> MoveLeft(int pos) {
  if (pos % (kRowSize + 1) == 0) return std::nullopt;
  return pos - 1;
}

std::optional<int> MoveRight(int pos) {
  if (pos % (kRowSize + 1) == kRowSize - 1) return std::nullopt;
  return pos + 1;
}

std::optional<int> MoveUp(int pos) {
  if (pos < kRowSize) return std::nullopt;
  return pos - kRowSize - 1;
}

std::optional<int> MoveDown(int pos) {
  if (pos + kRowSize >= kInputSize) return std::nullopt;
  return pos + kRowSize + 1;
}

enum Direction {
  LEFT,
  RIGHT,
  UP,
  DOWN,
};

std::optional<int> Move(int pos, const Direction& direction) {
  switch (direction) {
    case LEFT:
      return MoveLeft(pos);
      break;
    case RIGHT:
      return MoveRight(pos);
      break;
    case UP:
      return MoveUp(pos);
      break;
    case DOWN:
      return MoveDown(pos);
      break;
    default:
      std::cerr << "Invalid direction \n";
      throw std::invalid_argument("Invalid direction");
  }
}

std::optional<Direction> NextDirection(char c, const Direction& direction) {
  switch (c) {
    case '.':
    case 'S':
      return std::nullopt;
    case '-': {
      if (direction == RIGHT) {
        return {RIGHT};
      } else if (direction == LEFT) {
        return {LEFT};
      } else {
        return std::nullopt;
      }
    }
    case '|': {
      if (direction == UP) {
        return {UP};
      } else if (direction == DOWN) {
        return {DOWN};
      } else {
        return std::nullopt;
      }
    }
    case 'F': {
      if (direction == UP) {
        return {RIGHT};
      } else if (direction == LEFT) {
        return {DOWN};
      } else {
        return std::nullopt;
      }
    }
    case 'L': {
      if (direction == DOWN) {
        return {RIGHT};
      } else if (direction == LEFT) {
        return {UP};
      } else {
        return std::nullopt;
      }
    }
    case 'J': {
      if (direction == DOWN) {
        return {LEFT};
      } else if (direction == RIGHT) {
        return {UP};
      } else {
        return std::nullopt;
      }
    }
    case '7': {
      if (direction == UP) {
        return {LEFT};
      } else if (direction == RIGHT) {
        return {DOWN};
      } else {
        return std::nullopt;
      }
    }
    default:
      std::cerr << "Invalid character " << c << "\n";
      throw std::invalid_argument("Invalid character");
  }
}

std::optional<std::pair<Direction, std::unordered_set<int>>> Traverse(
    const std::string& input, int start_pos, const Direction& start_direction) {
  int pos = start_pos;
  std::unordered_set<int> loop = {pos};
  Direction direction = start_direction;
  while (true) {
    std::optional<int> next_pos = Move(pos, direction);
    if (!next_pos.has_value()) return std::nullopt;
    pos = next_pos.value();
    loop.insert(pos);
    if (pos == start_pos) break;
    std::optional<Direction> next_direction =
        NextDirection(input.at(pos), direction);
    if (!next_direction.has_value()) return std::nullopt;
    direction = next_direction.value();
  }
  return std::make_optional(std::make_pair(direction, loop));
}

char GetShape(Direction dir1, Direction dir2) {
  if ((dir1 == LEFT && dir2 == LEFT) || (dir1 == RIGHT && dir2 == RIGHT)) {
    return '-';
  }
  if ((dir1 == UP && dir2 == UP) || (dir1 == DOWN && dir2 == DOWN)) {
    return '|';
  }
  if ((dir1 == LEFT && dir2 == DOWN) || (dir1 == UP && dir2 == RIGHT)) {
    return 'J';
  }
  if ((dir1 == RIGHT && dir2 == DOWN) || (dir1 == UP && dir2 == LEFT)) {
    return 'L';
  }
  if ((dir1 == RIGHT && dir2 == UP) || (dir1 == DOWN && dir2 == LEFT)) {
    return 'F';
  }
  if ((dir1 == LEFT && dir2 == UP) || (dir1 == DOWN && dir2 == RIGHT)) {
    return '7';
  }
  std::cerr << "Invalid direction pair: " << dir1 << " and " << dir2 << '\n';
  throw std::invalid_argument("Invalid direction pair");
}

std::pair<char, std::unordered_set<int>> TraverseAll(const std::string& input,
                                                     int start_pos) {
  if (auto loop = Traverse(input, start_pos, LEFT); loop.has_value())
    return std::make_pair(GetShape(LEFT, loop->first), loop->second);
  if (auto loop = Traverse(input, start_pos, RIGHT); loop.has_value())
    return std::make_pair(GetShape(RIGHT, loop->first), loop->second);
  if (auto loop = Traverse(input, start_pos, UP); loop.has_value())
    return std::make_pair(GetShape(UP, loop->first), loop->second);
  if (auto loop = Traverse(input, start_pos, DOWN); loop.has_value())
    return std::make_pair(GetShape(DOWN, loop->first), loop->second);
  std::cerr << "Could not find a loop.";
  throw std::invalid_argument(input);
}

int FindStart(const std::string& input) {
  for (int pos = 0; pos < input.size(); ++pos) {
    if (input[pos] == 'S') {
      return pos;
    }
  }
  std::cerr << "Couldn't find starting point.";
  throw std::invalid_argument(input);
}

int CountInside(const std::string& input, const std::unordered_set<int>& loop,
                char start_char) {
  bool top_right_inside = false;
  bool bottom_right_inside = false;
  int count = 0;
  for (int pos = 0; pos < input.size(); ++pos) {
    char c = input[pos];
    if (c == 'S') c = start_char;
    if (c == '\n') continue;
    if (!loop.contains(pos)) {
      assert(bottom_right_inside == top_right_inside);
      if (bottom_right_inside) {
        ++count;
      }
    } else {
      switch (c) {
        case '|': {
          assert(top_right_inside == bottom_right_inside);
          top_right_inside = !top_right_inside;
          bottom_right_inside = !bottom_right_inside;
          break;
        }
        case 'L': {
          assert(top_right_inside == bottom_right_inside);
          top_right_inside = !top_right_inside;
          break;
        }
        case 'J': {
          if (top_right_inside == bottom_right_inside) {
            std::cerr << pos << " " << top_right_inside << " "
                      << bottom_right_inside << '\n';
            assert(false);
          }
          top_right_inside = !top_right_inside;
          break;
        }
        case '7': {
          if (top_right_inside == bottom_right_inside) {
            std::cerr << pos << " " << top_right_inside << " "
                      << bottom_right_inside << '\n';
            assert(false);
          }
          bottom_right_inside = !bottom_right_inside;
          break;
        }
        case 'F': {
          assert(top_right_inside == bottom_right_inside);
          bottom_right_inside = !bottom_right_inside;
          break;
        }
        case '-': {
          assert(top_right_inside != bottom_right_inside);
          break;
        }
        default:
          break;
      }
    }
  }
  return count;
}

std::string Run(const std::string& input) {
  // Your code goes here
  kInputSize = input.size();
  int start_pos = FindStart(input);
  const auto& [start_shape, loop] = TraverseAll(input, start_pos);
  int count_inside = CountInside(input, loop, start_shape);
  return std::to_string(count_inside);
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
