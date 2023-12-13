#include <chrono>
#include <iostream>
#include <span>
#include <string>

static constexpr int kFactor = 100;

bool IsVerticalReflection(const std::string& input, int min_pos, int n_rows,
                          int n_cols, int left) {
  int right = left + 1;
  bool smudge = false;
  while (left >= 0 && right < n_cols) {
    for (int row = 0; row < n_rows; ++row) {
      if (input[min_pos + left + (n_cols + 1) * row] !=
          input[min_pos + right + (n_cols + 1) * row]) {
        if (smudge) return false;
        smudge = true;
      }
    }
    --left;
    ++right;
  }
  return smudge;
}

bool IsHorizontalReflection(const std::string& input, int min_pos, int n_rows,
                            int n_cols, int up) {
  int down = up + 1;
  bool smudge = false;
  while (up >= 0 && down < n_rows) {
    for (int col = 0; col < n_cols; ++col) {
      if (input[min_pos + col + (n_cols + 1) * up] !=
          input[min_pos + col + (n_cols + 1) * down]) {
        if (smudge) return false;
        smudge = true;
      }
    }
    --up;
    ++down;
  }
  return smudge;
}

int GetScore(const std::string& input, int min_pos, int n_rows, int n_cols) {
  int result = 0;
  for (int col = 0; col < n_cols - 1; ++col) {
    if (IsVerticalReflection(input, min_pos, n_rows, n_cols, col))
      result += col + 1;
  }
  for (int row = 0; row < n_rows - 1; ++row) {
    if (IsHorizontalReflection(input, min_pos, n_rows, n_cols, row))
      result += kFactor * (row + 1);
  }
  return result;
}

int Parse(const std::string& input) {
  int n_rows = 0;
  int n_cols = 0;
  int min_pos = 0;
  int result = 0;
  bool first_char = true;
  for (int pos = 0; pos < input.size(); ++pos) {
    if (first_char && input[pos] == '\n') {
      result += GetScore(input, min_pos, n_rows, n_cols);
      min_pos = pos + 1;
      n_rows = 0;
      first_char = true;
      n_cols = 0;
      continue;
    }
    if (input[pos] == '\n') {
      if (n_cols == 0) n_cols = pos - min_pos;
      first_char = true;
      ++n_rows;
      continue;
    }
    first_char = false;
  }
  result += GetScore(input, min_pos, n_rows + 1, n_cols);
  return result;
}

std::string Run(const std::string& input) {
  // Your code goes here
  return std::to_string(Parse(input));
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
