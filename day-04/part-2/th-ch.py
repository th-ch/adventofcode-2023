from tool.runners.python import SubmissionPy

class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        cards = {}
        for line in s.splitlines():
            card, game = line.split(": ")
            card_id = int(card[5:])
            winning_numbers, numbers = game.split("|")
            winning = set(map(int, winning_numbers.split())) & set(map(int, numbers.split()))
            cards[card_id] = len(winning)

        scratchcards= {card_id: 1 for card_id in range(1, 1+len(cards))}
        for card_id in range(1, 1+len(cards)):
            for win in range(card_id+1, card_id+1+cards[card_id]):
                scratchcards[win] += scratchcards[card_id]

        return sum(scratchcards.values())


def test_th_ch():
    """
    Run `python -m pytest ./day-04/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
""".strip()
        )
        == 30
    )
