#include <chrono>
#include <iostream>
#include <span>
#include <stdexcept>
#include <string>

static constexpr int kSize = 100;

int GetColWeight(const std::string& input, int col) {
  int weight = kSize;
  int col_weight = 0;
  for (int row = 0; row < kSize; ++row) {
    switch (input[row * (kSize + 1) + col]) {
      case 'O': {
        col_weight += weight;
        --weight;
        break;
      }
      case '#': {
        weight = kSize - row - 1;
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
  return col_weight;
}

int GetTotalWeight(const std::string& input) {
  int total_weight = 0;
  for (int col = 0; col < kSize; ++col) {
    total_weight += GetColWeight(input, col);
  }
  return total_weight;
}

std::string Run(const std::string& input) {
  // Your code goes here
  return std::to_string(GetTotalWeight(input));
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
