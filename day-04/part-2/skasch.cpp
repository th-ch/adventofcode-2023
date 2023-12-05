#include <chrono>
#include <cstddef>
#include <iostream>
#include <ostream>
#include <set>
#include <span>
#include <sstream>
#include <string>
#include <vector>

class AggregatedCounts {
 public:
  int GetCount() { return current_count_; }
  void UpdateCount(int wins) {
    if (index_ + wins + 2 > delta_counts_.size()) {
      delta_counts_.resize(index_ + wins + 2, 0);
    }
    delta_counts_[index_ + 1] += GetCount();
    delta_counts_[index_ + wins + 1] -= GetCount();
  }
  void Increment() {
    ++index_;
    if (index_ == delta_counts_.size()) {
      delta_counts_.push_back(0);
    } else {
      current_count_ += delta_counts_[index_];
    }
  }

  AggregatedCounts() : delta_counts_() { delta_counts_.push_back(0); }

 private:
  int current_count_ = 1;
  int index_ = 0;
  std::vector<int> delta_counts_;
};

int GetCardCount(const std::string& line, AggregatedCounts& aggregated_counts) {
  std::set<int> winning_numbers = {};
  for (std::size_t pos = 10; pos < 39; pos += 3) {
    winning_numbers.insert(atoi(line.substr(pos, 2).c_str()));
  }
  int wins = 0;
  for (std::size_t pos = 42; pos < line.size(); pos += 3) {
    if (winning_numbers.contains(atoi(line.substr(pos, 2).c_str()))) {
      ++wins;
    }
  }
  int count = aggregated_counts.GetCount();
  if (wins > 0) {
    aggregated_counts.UpdateCount(wins);
  }
  aggregated_counts.Increment();
  return count;
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  int result = 0;
  AggregatedCounts aggregated_counts;
  for (std::string line; std::getline(iss, line);) {
    result += GetCardCount(line, aggregated_counts);
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
