#include <chrono>
#include <cstddef>
#include <functional>
#include <iostream>
#include <span>
#include <stdexcept>
#include <string>
#include <unordered_map>

static constexpr int kSize = 100;
static constexpr int kIterations = 1'000'000'000;

static std::hash<std::string> kHasher;

int ToPos(bool vertical, int index, int other) {
  if (vertical) {
    return other * (kSize + 1) + index;
  }
  return index * (kSize + 1) + other;
}

int GetTotalWeight(const std::string& input) {
  int total_weight = 0;
  for (int row = 0; row < kSize; ++row) {
    for (int col = 0; col < kSize; ++col) {
      if (input[ToPos(false, row, col)] == 'O') total_weight += kSize - row;
    }
  }
  return total_weight;
}

int GetEndCond(int direction, int other) {
  if (direction == 1) return other < kSize;
  return other >= 0;
}

int GetInitialOther(int direction) {
  if (direction == 1) return 0;
  return kSize - 1;
}

void MoveBase(std::string& input, int index, bool vertical, int direction) {
  int target_other = GetInitialOther(direction);
  for (int other = GetInitialOther(direction); GetEndCond(direction, other);
       other += direction) {
    switch (input[ToPos(vertical, index, other)]) {
      case 'O': {
        std::swap(input[ToPos(vertical, index, other)],
                  input[ToPos(vertical, index, target_other)]);
        target_other += direction;
        break;
      }
      case '#': {
        target_other = other + direction;
        break;
      }
      case '.': {
        break;
      }
      default: {
        throw std::invalid_argument("Invalid character");
      }
    }
  }
}

void MoveAllBase(std::string& input, bool vertical, int direction) {
  for (int index = 0; index < kSize; ++index) {
    MoveBase(input, index, vertical, direction);
  }
}

void MoveUp(std::string& input) { MoveAllBase(input, true, 1); }

void MoveDown(std::string& input) { MoveAllBase(input, true, -1); }

void MoveLeft(std::string& input) { MoveAllBase(input, false, 1); }

void MoveRight(std::string& input) { MoveAllBase(input, false, -1); }

void Cycle(std::string& input) {
  MoveUp(input);
  MoveLeft(input);
  MoveDown(input);
  MoveRight(input);
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::string input_copy = input;
  std::unordered_map<std::size_t, int> hash_reps;
  bool found_cycle = false;
  for (int rep = 1; rep < kIterations + 1; ++rep) {
    Cycle(input_copy);
    if (!found_cycle) {
      std::size_t hash = kHasher(input_copy);
      auto it = hash_reps.find(hash);
      if (it != hash_reps.end()) {
        int cycle_size = rep - it->second;
        rep += ((kIterations - rep) / cycle_size) * cycle_size;
        found_cycle = true;
      } else {
        hash_reps.insert({hash, rep});
      }
    }
  }
  return std::to_string(GetTotalWeight(input_copy));
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
