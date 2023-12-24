#include <chrono>
#include <iostream>
#include <span>
#include <sstream>
#include <string>

static constexpr __int128_t kMinTestArea = 200000000000000;
static constexpr __int128_t kMaxTestArea = 400000000000000;
static constexpr int kNHails = 300;

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

class Frac {
 public:
  Frac(__int128_t n) : num_(n), den_(1) {}
  Frac() : Frac(0) {}
  Frac(__int128_t n, __int128_t d) {
    if (d < 0) {
      n = -n;
      d = -d;
    }
    __int128_t g = gcd(n, d);
    num_ = n / g;
    den_ = d / g;
  }

  __int128_t n() const { return num_; }
  __int128_t d() const { return den_; }

  Frac operator+(const Frac& o) const {
    __int128_t g = gcd(d(), o.d());
    __int128_t d1 = d() / g;
    __int128_t d2 = o.d() / g;
    return Frac(n() * d2 + d1 * o.n(), g * d1 * d2);
  }
  Frac operator-() const { return Frac(-n(), d()); }
  Frac operator-(const Frac& o) const { return *this + (-o); }

  Frac operator*(const Frac& o) const {
    __int128_t g1 = gcd(n(), o.d());
    __int128_t g2 = gcd(o.n(), d());
    return Frac((n() / g1) * (o.n() / g2), (d() / g2) * (o.d() / g1));
  }
  Frac operator/(__int128_t k) const {
    __int128_t g = gcd(n(), k);
    return Frac(n() / g, d() * (k / g));
  }
  friend Frac operator/(__int128_t k, const Frac& o) {
    __int128_t g = gcd(k, o.n());
    return Frac((k / g) * o.d(), o.n() / g);
  }
  Frac operator/(const Frac& o) const { return *this * (1 / o); }

  Frac operator*(__int128_t k) {
    __int128_t g = gcd(k, d());
    return Frac((k / g) * n(), d() / g);
  }
  friend Frac operator*(__int128_t k, const Frac& o) { return o * k; }

  bool operator==(const Frac& o) const { return n() == o.n() && d() == o.d(); }
  auto operator<=>(const Frac& o) const { return n() * o.d() <=> d() * o.n(); }
  auto operator<=>(__int128_t k) const { return n() <=> d() * k; }
  friend auto operator<=>(__int128_t k, const Frac& o) {
    return k * o.d() <=> o.n();
  }

 private:
  __int128_t num_;
  __int128_t den_;
};

struct Vec {
  Frac x;
  Frac y;

  bool InTestArea() {
    return kMinTestArea <= x && x <= kMaxTestArea && kMinTestArea <= y &&
           y <= kMaxTestArea;
  }

  Vec operator+(const Vec& o) const { return {x + o.x, y + o.y}; }
  friend Vec operator*(Frac k, const Vec& v) { return {k * v.x, k * v.y}; }
};

struct Hail {
  Vec p;
  Vec v;

  std::optional<Vec> Intersect(const Hail& o) const {
    Frac delta = v.x * o.v.y - v.y * o.v.x;
    if (delta == 0) {
      return std::nullopt;
    }
    Frac a = o.v.y / delta * (o.p.x - p.x) - o.v.x / delta * (o.p.y - p.y);
    Frac b = v.y / delta * (o.p.x - p.x) - v.x / delta * (o.p.y - p.y);
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
