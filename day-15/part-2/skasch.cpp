#include <chrono>
#include <cstddef>
#include <iostream>
#include <span>
#include <string>
#include <unordered_map>

struct Node {
  int focal;
  std::string label;
  Node* next = nullptr;
  Node* prev = nullptr;
};

static constexpr int kNNodes = 4096;
static std::array<Node, kNNodes> kNodes;
static int kIndex = 0;

class Box {
 public:
  void Set(const std::string& label, int focal) {
    auto it = map_.find(label);
    if (it != map_.end()) {
      it->second->focal = focal;
    } else {
      kNodes[kIndex] = {.focal = focal, .label = label, .prev = tail_};
      if (tail_ == nullptr) {
        head_ = &kNodes[kIndex];
        tail_ = &kNodes[kIndex];
      } else {
        tail_->next = &kNodes[kIndex];
        tail_ = tail_->next;
      }
      ++kIndex;
      map_.insert({label, tail_});
    }
  }

  void Remove(const std::string& label) {
    auto it = map_.find(label);
    if (it == map_.end()) return;
    if (it->second == head_ && it->second == tail_) {
      head_ = nullptr;
      tail_ = nullptr;
    } else if (it->second == head_) {
      head_ = head_->next;
      head_->prev = nullptr;
    } else if (it->second == tail_) {
      tail_ = tail_->prev;
      tail_->next = nullptr;
    } else {
      it->second->prev->next = it->second->next;
      it->second->next->prev = it->second->prev;
    }
    map_.erase(it);
  }

  Node* GetHead() { return head_; }

  Box() : head_(nullptr), tail_(nullptr), map_() {}

 private:
  Node* head_;
  Node* tail_;
  std::unordered_map<std::string, Node*> map_;
};

static constexpr int kNBoxes = 256;
static std::array<Box, kNBoxes> kBoxes;

int GetScore() {
  int score = 0;
  for (int box_idx = 0; box_idx < kNBoxes; ++box_idx) {
    Node* node = kBoxes[box_idx].GetHead();
    int node_pos = 1;
    while (node != nullptr) {
      score += (box_idx + 1) * node_pos * node->focal;
      ++node_pos;
      node = node->next;
    }
  }
  return score;
}

std::string Run(const std::string& input) {
  // Your code goes here
  int label_start = 0;
  std::uint8_t code = 0;
  for (int pos = 0; pos < input.size(); ++pos) {
    switch (input[pos]) {
      case '\n':
      case ',':
        break;
      case '=':
        kBoxes[int(code)].Set(input.substr(label_start, pos - label_start),
                              input[pos + 1] - '0');
        code = 0;
        label_start = pos + 3;
        pos += 2;
        break;
      case '-':
        kBoxes[int(code)].Remove(input.substr(label_start, pos - label_start));
        code = 0;
        label_start = pos + 2;
        ++pos;
        break;
      default:
        code += input[pos];
        code *= 17;
        break;
    }
  }
  return std::to_string(GetScore());
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
