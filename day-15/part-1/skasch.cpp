#include <chrono>
#include <cstdint>
#include <iostream>
#include <span>
#include <string>

std::string Run(const std::string& input) {
  // Your code goes here
  int result = 0;
  std::uint8_t code = 0;
  for (char c : input) {
    switch (c) {
      case '\n':
        break;
      case ',':
        result += code;
        code = 0;
        break;
      default:
        code += c;
        code *= 17;
        break;
    }
  }
  return std::to_string(result + code);
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
