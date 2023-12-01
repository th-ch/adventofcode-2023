#include <chrono>
#include <iostream>
#include <span>
#include <sstream>
#include <string>

static constexpr int kBase = 2 << 5;  // > 27
static constexpr int kBase2 = kBase * kBase;
static constexpr int kBase3 = kBase2 * kBase;
static constexpr int kBase4 = kBase3 * kBase;

constexpr int ToRepr(char c) { return c - 'a' + 1; }

static constexpr int kOne =
    kBase2 * ToRepr('o') + kBase * ToRepr('n') + ToRepr('e');
static constexpr int kTwo =
    kBase2 * ToRepr('t') + kBase * ToRepr('w') + ToRepr('o');
static constexpr int kThree = kBase4 * ToRepr('t') + kBase3 * ToRepr('h') +
                              kBase2 * ToRepr('r') + kBase * ToRepr('e') +
                              ToRepr('e');
static constexpr int kFour = kBase3 * ToRepr('f') + kBase2 * ToRepr('o') +
                             kBase * ToRepr('u') + ToRepr('r');
static constexpr int kFive = kBase3 * ToRepr('f') + kBase2 * ToRepr('i') +
                             kBase * ToRepr('v') + ToRepr('e');
static constexpr int kSix =
    kBase2 * ToRepr('s') + kBase * ToRepr('i') + ToRepr('x');
static constexpr int kSeven = kBase4 * ToRepr('s') + kBase3 * ToRepr('e') +
                              kBase2 * ToRepr('v') + kBase * ToRepr('e') +
                              ToRepr('n');
static constexpr int kEight = kBase4 * ToRepr('e') + kBase3 * ToRepr('i') +
                              kBase2 * ToRepr('g') + kBase * ToRepr('h') +
                              ToRepr('t');
static constexpr int kNine = kBase3 * ToRepr('n') + kBase2 * ToRepr('i') +
                             kBase * ToRepr('n') + ToRepr('e');

bool IsInt(char c) { return '0' <= c && c <= '9'; }

int ToInt(char c) { return c - '0'; }

int ToDigit3(int hash3) {
  if (hash3 == kOne) return 1;
  if (hash3 == kTwo) return 2;
  if (hash3 == kSix) return 6;
  return 0;
}
int ToDigit4(int hash4) {
  if (hash4 == kFour) return 4;
  if (hash4 == kFive) return 5;
  if (hash4 == kNine) return 9;
  return 0;
}
int ToDigit5(int hash5) {
  if (hash5 == kThree) return 3;
  if (hash5 == kSeven) return 7;
  if (hash5 == kEight) return 8;
  return 0;
}

int UpdateHash(int hash, int val, int mod) {
  return ((hash % mod) << 5) /** kBase */ + val;
}

std::string Run(const std::string& input) {
  bool is_first_digit_found = false;
  int last_digit_found = 0;
  int result = 0;
  int hash3 = 0;
  int hash4 = 0;
  int hash5 = 0;
  for (char c : input) {
    if (c == '\n') {
      is_first_digit_found = false;
      result += last_digit_found;
      last_digit_found = 0;
      hash3 = 0;
      hash4 = 0;
      hash5 = 0;
      continue;
    }
    if (IsInt(c)) {
      last_digit_found = ToInt(c);
      hash3 = 0;
      hash4 = 0;
      hash5 = 0;
    } else {
      int repr = ToRepr(c);
      hash3 = UpdateHash(hash3, repr, kBase2);
      if (int d = ToDigit3(hash3); d != 0) {
        last_digit_found = d;
      }
      hash4 = UpdateHash(hash4, repr, kBase3);
      if (int d = ToDigit4(hash4); d != 0) {
        last_digit_found = d;
      }
      hash5 = UpdateHash(hash5, repr, kBase4);
      if (int d = ToDigit5(hash5); d != 0) {
        last_digit_found = d;
      }
    }
    if (last_digit_found != 0 && !is_first_digit_found) {
      is_first_digit_found = true;
      result += 10 * last_digit_found;
    }
  }
  result += last_digit_found;
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
