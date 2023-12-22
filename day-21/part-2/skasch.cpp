#include <chrono>
#include <cstdint>
#include <iostream>
#include <span>
#include <string>
#include <unordered_set>

static constexpr int kSize = 131;
static constexpr int kSteps = 26501365;

// Relative to S; assumed to be at the center of the grid
struct Pos {
  int x;
  int y;

  Pos operator+(const Pos& o) const { return {x + o.x, y + o.y}; }
  Pos operator*(int k) const { return {k * x, k * y}; }
  bool operator==(const Pos& o) const { return x == o.x && y == o.y; }

  friend std::ostream& operator<<(std::ostream& out, const Pos& pos) {
    return out << "(" << pos.x << "," << pos.y << ")";
  }
};

struct PosHash {
  size_t operator()(const Pos& pos) const {
    return std::hash<int>()(pos.x) ^ std::hash<int>()(pos.y);
  }
};

using PosMap = std::unordered_set<Pos, PosHash>;

static constexpr std::array<Pos, 4> kDirections = {Pos{1, 0}, Pos{-1, 0},
                                                   Pos{0, 1}, Pos{0, -1}};

bool IsValid(int pos, const std::string& input) {
  return 0 <= pos && pos < input.size() && input[pos] != '\n' &&
         input[pos] != '#';
}

int PosMod(int v, int m) { return (v % m + m) % m; }

int GetIndex(const Pos& pos, int base = kSize) {
  return PosMod(pos.y + (base - 1) / 2, base) * (base + 1) +
         PosMod(pos.x + (base - 1) / 2, base);
}

std::int64_t CountPos(const std::string& input, int side, int count) {
  PosMap even = {{Pos{0, 0}}};
  PosMap even_front = {{Pos{0, 0}}};
  PosMap odd;
  PosMap odd_front;
  std::int64_t c = 0;
  std::int64_t v1 = 0;
  std::int64_t v2 = 0;
  PosMap& starts = even;
  PosMap& starts_front = even_front;
  PosMap& ends = odd;
  PosMap& ends_front = odd_front;
  for (int step = 1; step <= side / 2 + 2 * side; ++step) {
    ends_front.clear();
    for (const Pos& start : starts_front) {
      for (const Pos& dir : kDirections) {
        Pos end = start + dir;
        if (input[GetIndex(end)] == '#') continue;
        if (ends.insert(end).second) {
          ends_front.insert(end);
        }
      }
    }
    if (step == side / 2)
      c = ends.size();
    else if (step == side / 2 + side)
      v1 = ends.size();
    std::swap(starts, ends);
    std::swap(starts_front, ends_front);
  }
  v2 = starts.size();
  std::int64_t a = (v2 + c) / 2 - v1;
  std::int64_t b = v1 - c - a;
  std::int64_t reps = (count - side / 2) / side;
  return a * reps * reps + b * reps + c;
}

std::string Run(const std::string& input) {
  // Your code goes here
  return std::to_string(CountPos(input, kSize, kSteps));
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
