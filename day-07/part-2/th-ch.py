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
    J = unique.pop("J", 0)
    # Five of a kind
    if len(unique)<=1:
        rank = 1
    # we know J is <= 3 from now on
    # Four of a kind
    elif len(unique)==2 and any(v+J==4 for v in unique.values()):
        rank = 2
    # we know J is <= 2 from now on
    # Full house
    elif len(unique)==2 and any(v+J==3 for v in unique.values()):
        rank = 3
    # Three of a kind
    elif len(unique)==3 and any(v+J==3 for v in unique.values()):
        rank = 4
    # we know J is <= 1 from now on
    # Two pairs
    elif (J==0 and len([v for v in unique.values() if v==2]) == 2) or (J==1 and any(v==2 for v in unique.values())):
        rank = 5
    # One pair
    elif any(v+J==2 for v in unique.values()):
        rank = 6
    # High card
    else:
        rank = 7

    return rank, *["AKQT98765432J".index(c) for c in hand]


def test_th_ch():
    """
    Run `python -m pytest ./day-07/part-2/th-ch.py` to test the submission.
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
        == 5905
    )

