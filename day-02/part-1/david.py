from tool.runners.python import SubmissionPy


class DavidSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        lines = s.split("\n")

        def is_game_possible(game_content: str) -> bool:
            game_content = game_content.replace(";", ",")
            for draw in game_content.split(", "):
                count, color = draw.split(" ")
                count = int(count)
                if color == 'red' and count > 12:
                    return False
                if color == 'green' and count > 13:
                    return False
                if color == 'blue' and count > 14:
                    return False
            return True

        result = 0
        for line in lines:
            game_id_str, game_content = line.split(": ")
            game_id = int(game_id_str[5:])
            if is_game_possible(game_content):
                result += game_id

        return result
