#include <algorithm>
#include <array>
#include <chrono>
#include <cstddef>
#include <iostream>
#include <span>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

static constexpr int kBase = 26;
static constexpr int kSize = kBase * kBase * kBase;
static std::array<std::unordered_set<int>, kSize + 1> kGraph;
static std::array<int, kSize + 1> kFrequencies;
static std::array<bool, kSize + 1> kVisited;
static std::unordered_set<int> kNodes;
static constexpr int kNodesToExplore = 10;
static constexpr int kBridges = 3;

int ToIndex(const std::string& label) {
  return kBase * kBase * (label[0] - 'a') + kBase * (label[1] - 'a') +
         label[2] - 'a' + 1;
}

std::string ToLabel(int index) {
  --index;
  std::string result;
  result.push_back(char(index / (kBase * kBase) + 'a'));
  result.push_back(char(((index / kBase) % kBase) + 'a'));
  result.push_back(char(index % kBase + 'a'));
  return result;
}

void Bfs(int start) {
  std::unordered_set<int> visited = {start};
  std::unordered_map<int, int> parents;
  std::vector<std::pair<int, int>> layer;
  for (int next : kGraph[start]) {
    layer.push_back(std::make_pair(next, start));
  }
  while (!layer.empty()) {
    std::vector<std::pair<int, int>> next_layer;
    for (const auto& [node, prev] : layer) {
      visited.insert(node);
      parents[node] = prev;
      for (int path = node; path != start; path = parents[path]) {
        ++kFrequencies[path];
      }
      for (int next : kGraph[node]) {
        if (visited.contains(next)) continue;
        next_layer.push_back(std::make_pair(next, node));
      }
    }
    std::swap(layer, next_layer);
  }
}

int ComponentSize(int start) {
  std::stack<int> stack;
  int size = 0;
  stack.push(start);
  while (!stack.empty()) {
    int node = stack.top();
    stack.pop();
    if (kVisited[node]) continue;
    kVisited[node] = true;
    ++size;
    for (int next : kGraph[node]) {
      if (kVisited[next]) continue;
      stack.push(next);
    }
  }
  return size;
}

int ParseLine(const std::string& line) {
  int idx1 = ToIndex(line);
  for (int pos = 5; pos < int(line.size()); pos += 4) {
    int idx2 = ToIndex(line.substr(pos));
    kGraph[idx1].insert(idx2);
    kGraph[idx2].insert(idx1);
    kNodes.insert(idx1);
    kNodes.insert(idx2);
  }
  return idx1;
}

std::vector<int> GetKLargestFrequencies(int k) {
  std::vector<int> k_largest;
  auto cmp = [](int idx1, int idx2) -> bool {
    return kFrequencies[idx1] > kFrequencies[idx2];
  };
  for (int node : kNodes) {
    k_largest.push_back(node);
    std::push_heap(k_largest.begin(), k_largest.end(), cmp);
    if (int(k_largest.size()) > k) {
      std::pop_heap(k_largest.begin(), k_largest.end(), cmp);
      k_largest.pop_back();
    }
  }
  return k_largest;
}

int FindOneBridge(int node, const std::vector<int>& nodes) {
  for (int node2 : nodes) {
    if (kGraph[node].contains(node2)) {
      return node2;
    }
  }
  throw std::invalid_argument("No bridge found");
}

void RemoveBridges(const std::vector<int>& nodes) {
  for (int node1 : nodes) {
    for (int node2 : nodes) {
      kGraph[node1].erase(node2);
    }
  }
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  for (std::string line; std::getline(iss, line);) {
    ParseLine(line);
  }
  int explored_nodes = 0;
  for (int idx : kNodes) {
    if (explored_nodes >= kNodesToExplore) break;
    Bfs(idx);
    ++explored_nodes;
  }
  std::vector<int> most_frequent = GetKLargestFrequencies(2 * kBridges);
  int first = most_frequent[0];
  int second = FindOneBridge(first, most_frequent);
  RemoveBridges(most_frequent);
  return std::to_string(ComponentSize(first) * ComponentSize(second));
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
