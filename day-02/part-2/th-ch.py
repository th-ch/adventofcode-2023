from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        result = 0
        for game in s.splitlines():
            is_impossible = False
            game_with_id, all_games = game.split(": ")
            game_id = int(game_with_id.replace("Game ", ""))
            min_blue, min_red, min_green = 0, 0, 0
            for played_game in all_games.split("; "):
                colors = played_game.split(", ")
                for played_color in colors:
                    nb, color = played_color.split()
                    if color == "blue":
                        min_blue = max(min_blue, int(nb))
                    if color == "green":
                        min_green = max(min_green, int(nb))
                    if color == "red":
                        min_red = max(min_red, int(nb))
            power = min_blue * min_green * min_red
            result += power

        return result


def test_th_ch():
    """
    Run `python -m pytest ./day-02/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
""".strip()
        )
        == 2286
    )
