#include <chrono>
#include <iostream>
#include <span>
#include <sstream>
#include <string>
#include <vector>

static constexpr int kMaxRowSize = 255;
static int kRowSize = kMaxRowSize;

// https://cp-algorithms.com/data_structures/fenwick.html
struct FenwickTree {
  int n_;
  std::vector<int> bit_;  // binary indexed tree
  std::vector<bool> inserted_;

  FenwickTree(int n) : n_(n), bit_(n, 0), inserted_(n, false) {}

  int sum(int r) {
    int ret = 0;
    for (; r >= 0; r = (r & (r + 1)) - 1) ret += bit_[r];
    return ret;
  }

  int sum(int l, int r) { return sum(l) - sum(r - 1); }

  void set(int idx) {
    if (inserted_[idx]) return;
    for (int i = idx; i < n_; i = i | (i + 1)) ++bit_[i];
    inserted_[idx] = true;
  }
};

static FenwickTree kGalaxyRows(kMaxRowSize);
static FenwickTree kGalaxyCols(kMaxRowSize);

void ParseInput(const std::string& input,
                std::vector<std::pair<int, int>>& galaxies,
                FenwickTree& galaxy_rows, FenwickTree& galaxy_cols) {
  int row = 0;
  for (int pos = 0; pos < input.size(); ++pos) {
    if (input[pos] == '\n') {
      if (kRowSize == kMaxRowSize) kRowSize = pos;
      ++row;
      continue;
    }
    if (input[pos] == '#') {
      int col = pos % (kRowSize + 1);
      galaxies.push_back({row, col});
      galaxy_rows.set(row);
      galaxy_cols.set(col);
    }
  }
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  std::vector<std::pair<int, int>> galaxies;
  ParseInput(input, galaxies, kGalaxyRows, kGalaxyCols);
  int result = 0;
  for (int idx1 = 0; idx1 < galaxies.size() - 1; ++idx1) {
    for (int idx2 = idx1 + 1; idx2 < galaxies.size(); ++idx2) {
      int r1 = galaxies[idx1].first;
      int r2 = galaxies[idx2].first;
      if (r1 > r2) std::swap(r1, r2);
      int c1 = galaxies[idx1].second;
      int c2 = galaxies[idx2].second;
      if (c1 > c2) std::swap(c1, c2);
      int count_galaxy_rows = kGalaxyRows.sum(r2, r1);
      int count_galaxy_cols = kGalaxyCols.sum(c2, c1);
      result +=
          2 * (c2 + r2 - c1 - r1 + 1) - count_galaxy_rows - count_galaxy_cols;
    }
  }
  return std::to_string(result);
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
