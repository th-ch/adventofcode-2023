from tool.runners.python import SubmissionPy

from collections import Counter


def eval_hand(hand_str: str):
    hand_counter = Counter(hand_str)
    if "J" in hand_counter:
        if hand_counter["J"] == 5:
            return 6
        jokers = hand_counter.pop("J")
        for c in hand_counter:
            hand_counter[c] += jokers
    if len(hand_counter) == 1:
        return 6
    if len(hand_counter) == 2:
        if 4 in hand_counter.values():
            return 5
        return 4
    if len(hand_counter) == 3:
        if 3 in hand_counter.values():
            return 3
        return 2
    if len(hand_counter) == 4:
        return 1
    return 0


def parse_card(char_):
    if char_ == "T":
        return 10
    if char_ == "J":
        return 1
    if char_ == "Q":
        return 12
    if char_ == "K":
        return 13
    if char_ == "A":
        return 14
    return int(char_)


def hand_cards_values(hand_str: str):
    return [parse_card(char_) for char_ in hand_str]


class YouyounSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        hands = []
        for line in s.splitlines():
            hand, bid = line.split(" ")
            bid = int(bid)
            hands.append((hand, bid))
        hands.sort(key=lambda x: (eval_hand(x[0]), hand_cards_values(x[0])))
        return sum(hands[i][1] * (i + 1) for i in range(len(hands)))


def test_youyoun():
    """
    Run `python -m pytest ./day-07/part-2/youyoun.py` to test the submission.
    """
    assert (
        YouyounSubmission().run(
            """32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483""".strip()
        )
        == 5905
    )
