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

int GetPower(const std::string& line) {
  Draw max_draw = {};
  for (auto pos = line.find(':') + 2; pos < line.size();) {
    int next_pos = line.find(' ', pos);
    int cnt = atoi(line.substr(pos, next_pos - pos).c_str());
    pos = next_pos + 1;
    switch (line[pos]) {
      case 'r': {
        max_draw.R = std::max(max_draw.R, cnt);
        pos += 5;
        break;
      }
      case 'g': {
        max_draw.G = std::max(max_draw.G, cnt);
        pos += 7;
        break;
      }
      case 'b': {
        max_draw.B = std::max(max_draw.B, cnt);
        pos += 6;
        break;
      }
      default: {
        std::cerr << "Failed to parse " << line << '\n';
      }
    }
  }
  return max_draw.R * max_draw.G * max_draw.B;
}

std::string Run(const std::string& input) {
  // Your code goes here
  int result = 0;
  std::istringstream iss(input);
  for (std::string line; std::getline(iss, line);) {
    result += GetPower(line);
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
