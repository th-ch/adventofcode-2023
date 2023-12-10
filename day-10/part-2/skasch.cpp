#include <chrono>
#include <iostream>
#include <optional>
#include <span>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

static constexpr int kRowSize = 140;
static int kInputSize;
using Loop = std::vector<bool>;
static Loop kLoop1((kRowSize + 1) * kRowSize, false);
static Loop kLoop2((kRowSize + 1) * kRowSize, false);
static Loop kLoop3((kRowSize + 1) * kRowSize, false);

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
      throw std::invalid_argument("Invalid direction");
  }
}

std::optional<Direction> NextDirection(char c, const Direction& direction) {
  switch (c) {
    case '.':
    case 'S':
      return std::nullopt;
    case '-': {
      switch (direction) {
        case LEFT:
        case RIGHT:
          return std::make_optional(direction);
        default:
          return std::nullopt;
      }
    }
    case '|': {
      switch (direction) {
        case UP:
        case DOWN:
          return std::make_optional(direction);
        default:
          return std::nullopt;
      }
    }
    case 'F': {
      switch (direction) {
        case UP:
          return {RIGHT};
        case LEFT:
          return {DOWN};
        default:
          return std::nullopt;
      }
    }
    case 'L': {
      switch (direction) {
        case DOWN:
          return {RIGHT};
        case LEFT:
          return {UP};
        default:
          return std::nullopt;
      }
    }
    case 'J': {
      switch (direction) {
        case DOWN:
          return {LEFT};
        case RIGHT:
          return {UP};
        default:
          return std::nullopt;
      }
    }
    case '7': {
      switch (direction) {
        case UP:
          return {LEFT};
        case RIGHT:
          return {DOWN};
        default:
          return std::nullopt;
      }
    }
    default:
      throw std::invalid_argument("Invalid character");
  }
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
  throw std::invalid_argument("Invalid direction pair");
}

std::optional<char> Traverse(const std::string& input, int start_pos,
                             const Direction& start_direction, Loop& loop) {
  int pos = start_pos;
  loop[pos] = true;
  Direction direction = start_direction;
  while (true) {
    std::optional<int> next_pos = Move(pos, direction);
    if (!next_pos.has_value()) return std::nullopt;
    pos = next_pos.value();
    loop[pos] = true;
    if (pos == start_pos) break;
    std::optional<Direction> next_direction =
        NextDirection(input.at(pos), direction);
    if (!next_direction.has_value()) return std::nullopt;
    direction = next_direction.value();
  }
  return GetShape(start_direction, direction);
}

using TraverseResult = std::pair<char, const Loop&>;

TraverseResult TraverseAll(const std::string& input, int start_pos) {
  if (auto start_shape = Traverse(input, start_pos, LEFT, kLoop1);
      start_shape.has_value())
    return std::make_pair(*start_shape, kLoop1);
  if (auto start_shape = Traverse(input, start_pos, RIGHT, kLoop2);
      start_shape.has_value())
    return std::make_pair(*start_shape, kLoop2);
  if (auto start_shape = Traverse(input, start_pos, UP, kLoop3);
      start_shape.has_value())
    return std::make_pair(*start_shape, kLoop3);
  throw std::invalid_argument(input);
}

int FindStart(const std::string& input) {
  for (int pos = 0; pos < input.size(); ++pos) {
    if (input[pos] == 'S') {
      return pos;
    }
  }
  throw std::invalid_argument(input);
}

int CountInside(const std::string& input, const TraverseResult& result) {
  bool top_right_inside = false;
  bool bottom_right_inside = false;
  int count = 0;
  for (int pos = 0; pos < input.size(); ++pos) {
    char c = input[pos];
    if (c == 'S') c = result.first;
    if (c == '\n') continue;
    if (!result.second[pos]) {
      if (bottom_right_inside) ++count;
      continue;
    }
    switch (c) {
      case '|': {
        top_right_inside = !top_right_inside;
        bottom_right_inside = !bottom_right_inside;
        break;
      }
      case 'L':
      case 'J': {
        top_right_inside = !top_right_inside;
        break;
      }
      case '7':
      case 'F': {
        bottom_right_inside = !bottom_right_inside;
        break;
      }
      default:
        break;
    }
  }
  return count;
}

std::string Run(const std::string& input) {
  // Your code goes here
  kInputSize = input.size();
  int start_pos = FindStart(input);
  return std::to_string(CountInside(input, TraverseAll(input, start_pos)));
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
