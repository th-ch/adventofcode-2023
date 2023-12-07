from tool.runners.python import SubmissionPy

from collections import Counter


class ThChSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        hands = []
        bid_by_hand = {}
        for line in s.splitlines():
            hand, bid = line.split()
            hands.append(hand)
            bid_by_hand[hand] = int(bid)

        sorted_hands = sorted(hands, key=rank, reverse=True)
        return sum(bid_by_hand[hand] * (i+1) for i, hand in enumerate(sorted_hands))


def rank(hand):
    # rank 1 is the strongest
    unique = Counter(hand)
    if len(unique)==1:
        rank = 1
    elif len(unique)==2 and any(v==4 for v in unique.values()):
        rank = 2
    elif len(unique)==2 and any(v==3 for v in unique.values()):
        rank = 3
    elif len(unique)==3 and any(v==3 for v in unique.values()):
        rank = 4
    elif len([v for v in unique.values() if v==2]) == 2:
        rank = 5
    elif len([v for v in unique.values() if v==2]) == 1:
        rank = 6
    else:
        rank = 7

    return rank, *["AKQJT98765432".index(c) for c in hand]


def test_th_ch():
    """
    Run `python -m pytest ./day-07/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
""".strip()
        )
        == 6440
    )
