#include <chrono>
#include <cstdint>
#include <iostream>
#include <span>
#include <sstream>
#include <string>

static constexpr std::int64_t kMinTestArea = 200000000000000;
static constexpr std::int64_t kMaxTestArea = 400000000000000;
static constexpr int kNHails = 300;

struct Vec {
  long double x;
  long double y;

  bool InTestArea() {
    return kMinTestArea <= x && x <= kMaxTestArea && kMinTestArea <= y &&
           y <= kMaxTestArea;
  }

  Vec operator+(const Vec& o) const { return {x + o.x, y + o.y}; }
  friend Vec operator*(long double k, const Vec& v) {
    return {k * v.x, k * v.y};
  }
};

struct Hail {
  Vec p;
  Vec v;

  std::optional<Vec> Intersect(const Hail& o) const {
    long double delta = v.x * o.v.y - v.y * o.v.x;
    if (delta == 0) {
      return std::nullopt;
    }
    long double a =
        o.v.y / delta * (o.p.x - p.x) - o.v.x / delta * (o.p.y - p.y);
    long double b = v.y / delta * (o.p.x - p.x) - v.x / delta * (o.p.y - p.y);
    if (a < 0.0 || b < 0.0) {
      return std::nullopt;
    }
    return p + a * v;
  }
};

static std::array<Hail, kNHails> kHails;

void ParseLine(const std::string& line, int index) {
  int left = 0;
  int right = line.find(',', left);
  Hail& hail = kHails[index];
  hail.p.x = std::atoll(line.substr(left, right - left).c_str());
  left = right + 2;
  right = line.find(',', left);
  hail.p.y = std::atoll(line.substr(left, right - left).c_str());
  left = line.find('@', right) + 2;
  right = line.find(',', left);
  hail.v.x = std::atoll(line.substr(left, right - left).c_str());
  left = right + 2;
  right = line.find(',', left);
  hail.v.y = std::atoll(line.substr(left, right - left).c_str());
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  int index = 0;
  for (std::string line; std::getline(iss, line); ++index) {
    ParseLine(line, index);
  }
  int result = 0;
  for (int i1 = 1; i1 < index; ++i1) {
    for (int i2 = 0; i2 < i1; ++i2) {
      auto intersect = kHails[i1].Intersect(kHails[i2]);
      if (!intersect.has_value()) {
        continue;
      }
      if (!intersect->InTestArea()) {
        continue;
      }
      ++result;
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
