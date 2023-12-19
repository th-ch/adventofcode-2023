#include <chrono>
#include <cstdint>
#include <iostream>
#include <span>
#include <sstream>
#include <stdexcept>
#include <string>
#include <utility>

struct Pos {
  std::int64_t x;
  std::int64_t y;

  Pos operator+(const Pos& o) { return {x + o.x, y + o.y}; }
  void operator+=(const Pos& o) {
    x += o.x;
    y += o.y;
  }
  Pos operator*(std::int64_t o) { return {x * o, y * o}; }
};

static constexpr Pos kLeft = {-1, 0};
static constexpr Pos kRight = {1, 0};
static constexpr Pos kUp = {0, -1};
static constexpr Pos kDown = {0, 1};

std::pair<Pos, std::int64_t> ParseLine(const std::string& line) {
  int second_space = line.find(' ', 2);
  std::int64_t steps = std::stoi(line.substr(second_space + 3, 5), nullptr, 16);
  switch (line[second_space + 8]) {
    case '0': {
      return {kRight, steps};
    }
    case '1': {
      return {kDown, steps};
    }
    case '2': {
      return {kLeft, steps};
    }
    case '3': {
      return {kUp, steps};
    }
    default: {
      throw std::invalid_argument("Invalid direction.");
    }
  }
}
std::int64_t VectorProduct(const Pos& pos1, const Pos& pos2) {
  return pos1.x * pos2.y - pos2.x * pos1.y;
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  Pos pos = {0, 0};
  std::int64_t res = 0;
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
