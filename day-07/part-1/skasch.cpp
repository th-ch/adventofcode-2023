#include <algorithm>
#include <chrono>
#include <cstdint>
#include <iostream>
#include <span>
#include <sstream>
#include <stdexcept>
#include <string>
#include <unordered_map>

std::int8_t GetCardValue(char c) {
  switch (c) {
    case '2':
    case '3':
    case '4':
    case '5':
    case '6':
    case '7':
    case '8':
    case '9':
      return c - '2';
    case 'T':
      return 8;
    case 'J':
      return 9;
    case 'Q':
      return 10;
    case 'K':
      return 11;
    case 'A':
      return 12;
    default:
      std::cerr << "Invalid character " << c;
      throw std::invalid_argument("Invalid character");
  }
}

enum HandType {
  HighCard,
  OnePair,
  TwoPairs,
  ThreeOfAKind,
  FullHouse,
  FourOfAKind,
  FiveOfAKind
};

using Hand = std::array<std::int8_t, 5>;

HandType GetHandType(const Hand& hand) {
  std::unordered_map<std::int8_t, std::int8_t> card_counts;
  for (std::int8_t v : hand) ++card_counts[v];
  HandType hand_type = HandType::HighCard;
  for (const auto& [_, cnt] : card_counts) {
    switch (cnt) {
      case 1:
        break;
      case 2: {
        switch (hand_type) {
          case HandType::HighCard:
            hand_type = HandType::OnePair;
            break;
          case HandType::OnePair:
            hand_type = HandType::TwoPairs;
            break;
          case HandType::ThreeOfAKind:
            hand_type = HandType::FullHouse;
            break;
          default:
            std::cerr << "Impossible combination: hand is " << hand_type
                      << ", current card has " << cnt << " copies.\n";
            throw std::invalid_argument("Impossible hand");
        }
        break;
      }
      case 3: {
        switch (hand_type) {
          case HandType::HighCard:
            hand_type = HandType::ThreeOfAKind;
            break;
          case HandType::OnePair:
            hand_type = HandType::FullHouse;
            break;
          default:
            std::cerr << "Impossible combination: hand is " << hand_type
                      << ", current card has " << cnt << " copies.\n";
            throw std::invalid_argument("Impossible hand");
        }
        break;
      }
      case 4: {
        hand_type = HandType::FourOfAKind;
        break;
      }
      case 5: {
        hand_type = HandType::FiveOfAKind;
        break;
      }
    }
  }
  return hand_type;
}

struct HandAndBid {
  Hand hand;
  HandType hand_type;
  int bid;
};

bool CompareHands(const HandAndBid& left, const HandAndBid& right) {
  if (left.hand_type != right.hand_type)
    return left.hand_type < right.hand_type;
  return left.hand < right.hand;
}

HandAndBid ParseLine(const std::string& line) {
  Hand hand = {GetCardValue(line[0]), GetCardValue(line[1]),
               GetCardValue(line[2]), GetCardValue(line[3]),
               GetCardValue(line[4])};
  return {.hand = hand,
          .hand_type = GetHandType(hand),
          .bid = atoi(line.substr(6).c_str())};
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  std::vector<HandAndBid> hands;
  for (std::string line; std::getline(iss, line);) {
    hands.push_back(ParseLine(line));
  }
  std::sort(hands.begin(), hands.end(), CompareHands);
  int winnings = 0;
  for (int idx = 0; idx < hands.size(); ++idx) {
    winnings += (idx + 1) * hands[idx].bid;
  }
  return std::to_string(winnings);
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
