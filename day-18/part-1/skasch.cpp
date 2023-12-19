#include <chrono>
#include <iostream>
#include <span>
#include <sstream>
#include <stdexcept>
#include <string>
#include <utility>

struct Pos {
  int x;
  int y;

  Pos operator+(const Pos& o) { return {x + o.x, y + o.y}; }
  void operator+=(const Pos& o) {
    x += o.x;
    y += o.y;
  }
  Pos operator*(int o) { return {x * o, y * o}; }
};

static constexpr Pos kLeft = {-1, 0};
static constexpr Pos kRight = {1, 0};
static constexpr Pos kUp = {0, -1};
static constexpr Pos kDown = {0, 1};

std::pair<Pos, int> ParseLine(const std::string& line) {
  int second_space = line.find(' ', 2);
  int steps = std::atoi(line.substr(2, second_space - 2).c_str());
  switch (line[0]) {
    case 'L': {
      return {kLeft, steps};
    }
    case 'R': {
      return {kRight, steps};
    }
    case 'U': {
      return {kUp, steps};
    }
    case 'D': {
      return {kDown, steps};
    }
    default: {
      throw std::invalid_argument("Invalid direction.");
    }
  }
}
int VectorProduct(const Pos& pos1, const Pos& pos2) {
  return pos1.x * pos2.y - pos2.x * pos1.y;
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  Pos pos = {0, 0};
  int res = 0;
  for (std::string line; std::getline(iss, line);) {
    auto [direction, steps] = ParseLine(line);
    res += steps + VectorProduct(pos, pos + direction * steps);
    pos += direction * steps;
  }
  return std::to_string(std::abs(res) / 2 + 1);
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
