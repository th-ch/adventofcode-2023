#include <array>
#include <chrono>
#include <iostream>
#include <ostream>
#include <queue>
#include <span>
#include <sstream>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

static constexpr int kNBoxes = 2048;

struct Interval {
  int l;
  int r;
};

struct Box {
  Interval x;
  Interval y;
  Interval z;

  friend std::ostream& operator<<(std::ostream& out, const Box& v) {
    return out << "Box(x=" << v.x.l << ".." << v.x.r << ",y=" << v.y.l << ".."
               << v.y.r << ",z=" << v.z.l << ".." << v.z.r << ")";
  };
};

static std::array<Box, kNBoxes> kBoxes;

void ParseLine(const std::string& input, int index) {
  Box& box = kBoxes[index];
  int left = 0;
  int right = input.find(',');
  box.x.l = std::atoi(input.substr(left, right).c_str());
  left = right + 1;
  right = input.find(',', left);
  box.y.l = std::atoi(input.substr(left, right).c_str());
  left = right + 1;
  right = input.find('~', left);
  box.z.l = std::atoi(input.substr(left, right).c_str());
  left = right + 1;
  right = input.find(',', left);
  box.x.r = std::atoi(input.substr(left, right).c_str());
  left = right + 1;
  right = input.find(',', left);
  box.y.r = std::atoi(input.substr(left, right).c_str());
  left = right + 1;
  box.z.r = std::atoi(input.substr(left).c_str());
}

bool RangeOverlap(const Interval& i1, const Interval& i2) {
  return std::max(i1.l, i2.l) <= std::min(i1.r, i2.r);
}

bool RectOverlap(const Interval& x1, const Interval& y1, const Interval& x2,
                 const Interval& y2) {
  return RangeOverlap(x1, x2) && RangeOverlap(y1, y2);
}

bool Intersects(int idx1, int idx2) {
  return RectOverlap(kBoxes[idx1].x, kBoxes[idx1].y, kBoxes[idx2].x,
                     kBoxes[idx2].y);
}

using Pq =
    std::priority_queue<int, std::vector<int>, decltype([](int l, int r) {
                          return kBoxes[l].z.l > kBoxes[r].z.l ||
                                 (kBoxes[l].z.l == kBoxes[r].z.l &&
                                  kBoxes[l].z.r > kBoxes[r].z.r);
                        })>;

template <typename T>
T& Get(std::vector<T>& v, int idx) {
  if (v.size() <= idx) v.resize(idx + 1);
  return v[idx];
}

using SupportMap = std::unordered_map<int, std::vector<int>>;

std::pair<SupportMap, SupportMap> Stack(Pq& pq) {
  SupportMap supports;
  SupportMap supported_by;
  std::vector<std::vector<int>> starts;
  std::vector<std::vector<int>> ends;
  std::unordered_set<int> currents;
  while (!pq.empty()) {
    int index = pq.top();
    currents.clear();
    pq.pop();
    int z = kBoxes[index].z.l;
    for (; z > 0; --z) {
      for (int start : Get(starts, z)) currents.insert(start);
      for (int end : Get(ends, z)) currents.erase(end);
      bool intersects = false;
      for (int current : currents) {
        if (Intersects(index, current)) {
          intersects = true;
          supports[current].push_back(index);
          supported_by[index].push_back(current);
        }
      }
      if (!intersects) continue;
      break;
    }
    kBoxes[index].z.r -= kBoxes[index].z.l - z - 1;
    kBoxes[index].z.l = z + 1;
    Get(starts, kBoxes[index].z.r).push_back(index);
    Get(ends, kBoxes[index].z.l - 1).push_back(index);
  }
  return {supports, supported_by};
}

int CountFall(int index, SupportMap& supports, SupportMap& supported_by) {
  std::unordered_set<int> disintegrated = {index};
  std::unordered_set<int> layer(supports[index].begin(), supports[index].end());
  while (!layer.empty()) {
    std::unordered_set<int> next_layer;
    for (int l : layer) {
      bool can_disintegrate = true;
      for (int s : supported_by[l]) {
        if (!disintegrated.contains(s)) {
          can_disintegrate = false;
          break;
        }
      }
      if (!can_disintegrate) continue;
      disintegrated.insert(l);
      for (int n : supports[l]) next_layer.insert(n);
    }
    std::swap(layer, next_layer);
  }
  return disintegrated.size() - 1;
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  Pq pq;
  int index = 0;
  for (std::string line; std::getline(iss, line);) {
    ParseLine(line, index);
    pq.push(index);
    ++index;
  }
  auto [supports, supported_by] = Stack(pq);
  int result = 0;
  for (int idx = 0; idx < index; ++idx) {
    result += CountFall(idx, supports, supported_by);
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
