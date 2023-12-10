#include <chrono>
#include <iostream>
#include <optional>
#include <span>
#include <stdexcept>
#include <string>

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

std::optional<int> Traverse(const std::string& input, int start_pos,
                            const Direction& start_direction) {
  int steps = 0;
  int pos = start_pos;
  Direction direction = start_direction;
  while (true) {
    std::optional<int> next_pos = Move(pos, direction);
    if (!next_pos.has_value()) return std::nullopt;
    pos = next_pos.value();
    if (pos == start_pos) break;
    std::optional<Direction> next_direction =
        NextDirection(input.at(pos), direction);
    if (!next_direction.has_value()) return std::nullopt;
    direction = next_direction.value();
    ++steps;
  }
  return steps;
}

int TraverseAll(const std::string& input, int start_pos) {
  if (std::optional<int> steps = Traverse(input, start_pos, LEFT);
      steps.has_value())
    return *steps;
  if (std::optional<int> steps = Traverse(input, start_pos, RIGHT);
      steps.has_value())
    return *steps;
  if (std::optional<int> steps = Traverse(input, start_pos, UP);
      steps.has_value())
    return *steps;
  if (std::optional<int> steps = Traverse(input, start_pos, DOWN);
      steps.has_value())
    return *steps;
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

std::string Run(const std::string& input) {
  // Your code goes here
  kInputSize = input.size();
  int start_pos = FindStart(input);
  return std::to_string((TraverseAll(input, start_pos) + 1) / 2);
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
