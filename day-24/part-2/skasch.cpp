#include <chrono>
#include <cstddef>
#include <iostream>
#include <optional>
#include <span>
#include <sstream>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <unordered_set>

static constexpr std::int64_t kMaxVelocity = 1024;
static constexpr int kNHails = 300;
static std::unordered_map<int, std::vector<int>> kIndexesByVx;
static std::unordered_map<int, std::vector<int>> kIndexesByVy;
static std::unordered_map<int, std::vector<int>> kIndexesByVz;

__int128_t gcd(__int128_t a, __int128_t b) {
  if (b < 0) b = -b;
  if (a < 0) a = -a;
  if (a < b) std::swap(a, b);
  if (b == 0) return a;
  __int128_t r;
  while (a % b > 0) {
    r = a % b;
    a = b;
    b = r;
  }
  return b;
}

struct Vec {
  __int128_t x;
  __int128_t y;
  __int128_t z;

  Vec operator+(const Vec& o) const { return {x + o.x, y + o.y, z + o.z}; }
  Vec operator-() const { return {-x, -y, -z}; }
  Vec operator-(const Vec& o) const { return *this + (-o); }
  Vec operator*(__int128_t k) const { return {k * x, k * y, k * z}; }
  friend Vec operator*(__int128_t k, const Vec& v) { return v * k; }
  __int128_t operator*(const Vec& o) const {
    return x * o.x + y * o.y + z * o.z;
  }
  Vec operator^(const Vec& o) const {
    return {y * o.z - z * o.y, z * o.x - x * o.z, x * o.y - y * o.x};
  }
};

struct Hail {
  Vec p;
  Vec v;
};

std::optional<__int128_t> solve(__int128_t x11, __int128_t x12, __int128_t x21,
                                __int128_t x22, __int128_t b1, __int128_t b2) {
  __int128_t delta = x11 * x22 - x21 * x12;
  if (delta == 0) return std::nullopt;
  return (x22 * b1 - x12 * b2) / delta;
}

static std::array<Hail, kNHails> kHails;

void ParseLine(const std::string& line, int index) {
  int left = 0;
  int right = line.find(',', left);
  Hail& hail = kHails[index];
  hail.p.x = std::atoll(line.substr(left, right - left).c_str());
  left = right + 2;
  right = line.find(',', left);
  hail.p.y = std::atoll(line.substr(left, right - left).c_str());
  left = right + 2;
  right = line.find(' ', left);
  hail.p.z = std::atoll(line.substr(left, right - left).c_str());
  left = right + 3;
  right = line.find(',', left);
  hail.v.x = std::atoll(line.substr(left, right - left).c_str());
  kIndexesByVx[hail.v.x].push_back(index);
  left = right + 2;
  right = line.find(',', left);
  hail.v.y = std::atoll(line.substr(left, right - left).c_str());
  kIndexesByVy[hail.v.y].push_back(index);
  hail.v.z =
      std::atoll(line.substr(right + 2, line.size() - right - 2).c_str());
  kIndexesByVz[hail.v.z].push_back(index);
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  int index = 0;
  for (std::string line; std::getline(iss, line); ++index) {
    ParseLine(line, index);
  }
  std::unordered_set<int> invalid_vx;
  std::unordered_set<int> invalid_vy;
  std::unordered_set<int> invalid_vz;
  for (const auto& [ref_vx, indexes] : kIndexesByVx) {
    if (indexes.size() == 1) continue;
    invalid_vx.insert(ref_vx);
    for (std::size_t ii1 = 1; ii1 < indexes.size(); ++ii1) {
      for (std::size_t ii2 = 0; ii2 < ii1; ++ii2) {
        for (int vx = -kMaxVelocity; vx <= kMaxVelocity; ++vx) {
          if (vx == ref_vx) continue;
          Vec p1 = kHails[indexes[ii1]].p;
          Vec p2 = kHails[indexes[ii2]].p;
          if ((p1.x - p2.x) % (vx - ref_vx) != 0) {
            invalid_vx.insert(vx);
          }
        }
      }
    }
  }
  for (const auto& [ref_vy, indexes] : kIndexesByVy) {
    if (indexes.size() == 1) continue;
    invalid_vy.insert(ref_vy);
    for (std::size_t ii1 = 1; ii1 < indexes.size(); ++ii1) {
      for (std::size_t ii2 = 0; ii2 < ii1; ++ii2) {
        for (int vy = -kMaxVelocity; vy <= kMaxVelocity; ++vy) {
          if (vy == ref_vy) continue;
          Vec p1 = kHails[indexes[ii1]].p;
          Vec p2 = kHails[indexes[ii2]].p;
          if ((p1.y - p2.y) % (vy - ref_vy) != 0) {
            invalid_vy.insert(vy);
          }
        }
      }
    }
  }
  for (const auto& [ref_vz, indexes] : kIndexesByVz) {
    if (indexes.size() == 1) continue;
    invalid_vz.insert(ref_vz);
    for (std::size_t ii1 = 1; ii1 < indexes.size(); ++ii1) {
      for (std::size_t ii2 = 0; ii2 < ii1; ++ii2) {
        for (int vz = -kMaxVelocity; vz <= kMaxVelocity; ++vz) {
          if (vz == ref_vz) continue;
          Vec p1 = kHails[indexes[ii1]].p;
          Vec p2 = kHails[indexes[ii2]].p;
          if ((p1.z - p2.z) % (vz - ref_vz) != 0) {
            invalid_vz.insert(vz);
          }
        }
      }
    }
  }
  Vec p1 = kHails[0].p;
  Vec v1 = kHails[0].v;
  Vec p2 = kHails[1].p;
  Vec v2 = kHails[1].v;
  for (int vx = -kMaxVelocity; vx < kMaxVelocity; ++vx) {
    if (invalid_vx.contains(vx)) continue;
    for (int vy = -kMaxVelocity; vy < kMaxVelocity; ++vy) {
      if (invalid_vy.contains(vy)) continue;
      auto solution = solve(v1.x - vx, -v2.x + vx, v1.y - vy, -v2.y + vy,
                            p2.x - p1.x, p2.y - p1.y);
      if (!solution.has_value()) continue;
      __int128 t1 = *solution;
      for (int vz = -kMaxVelocity; vz < kMaxVelocity; ++vz) {
        if (invalid_vz.contains(vz)) continue;
        Vec v(vx, vy, vz);
        Vec p = p1 + t1 * (v1 - v);
        return std::to_string(std::int64_t(p.x + p.y + p.z));
      }
    }
  }
  throw std::invalid_argument("No solution found.");
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
