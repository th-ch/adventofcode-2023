#include <chrono>
#include <iostream>
#include <span>
#include <string>
#include <unordered_set>

static constexpr int kSize = 131;
static constexpr std::array<int, 4> kDirections = {1, -1, kSize + 1,
                                                   -kSize - 1};
static constexpr int kSteps = 64;

bool IsValid(int pos, const std::string& input) {
  return 0 <= pos && pos < input.size() && input[pos] != '\n' &&
         input[pos] != '#';
}

int CountPosAfter(const std::string& input,
                  const std::unordered_set<int>& starts, int steps) {
  if (steps == 0) return starts.size();
  std::unordered_set<int> ends;
  for (int start : starts) {
    for (int dir : kDirections) {
      int end = start + dir;
      if (!IsValid(end, input)) continue;
      ends.insert(end);
    }
  }
  return CountPosAfter(input, ends, steps - 1);
}

std::string Run(const std::string& input) {
  // Your code goes here
  int start = 0;
  for (; start < input.size(); ++start) {
    if (input[start] == 'S') break;
  }
  return std::to_string(CountPosAfter(input, {start}, kSteps));
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
