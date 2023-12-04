#include <chrono>
#include <iostream>
#include <span>
#include <sstream>
#include <string>

struct Draw {
  int R;
  int G;
  int B;
};

static constexpr Draw kMaxDraw = {.R = 12, .G = 13, .B = 14};

bool IsPossible(const std::string& line) {
  Draw draw = {};
  for (auto pos = line.find(':') + 2; pos < line.size();) {
    if (line[pos] == ';') {
      draw = Draw();
      pos += 2;
      continue;
    }
    if (line[pos] == ',') pos += 2;
    if (pos >= line.size()) break;
    int next_pos = line.find(' ', pos);
    int cnt = atoi(line.substr(pos, next_pos - pos).c_str());
    pos = next_pos + 1;
    switch (line[pos]) {
      case 'r': {
        draw.R += cnt;
        if (draw.R > kMaxDraw.R) return false;
        pos += 3;
        break;
      }
      case 'g': {
        draw.G += cnt;
        if (draw.G > kMaxDraw.G) return false;
        pos += 5;
        break;
      }
      case 'b': {
        draw.B += cnt;
        if (draw.B > kMaxDraw.B) return false;
        pos += 4;
        break;
      }
      default: {
        std::cerr << "Failed to parse " << line << '\n';
      }
    }
  }
  return true;
}

std::string Run(const std::string& input) {
  // Your code goes here
  int id = 1;
  int result = 0;
  std::istringstream iss(input);
  for (std::string line; std::getline(iss, line); ++id) {
    if (IsPossible(line)) {
      result += id;
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
