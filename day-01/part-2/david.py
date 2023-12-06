from tool.runners.python import SubmissionPy

class TrieNode:
    def __init__(self, value=None):
        self.children: dict[str, TrieNode] = dict()
        self.value: str = value

    def _get_or_create_child(self, letter: str):
        if letter not in self.children:
            child = TrieNode()
            self.children[letter] = child
        
        return self.children[letter]
    
    def insert_word(self, word: str, value: str):
        node: TrieNode = self
        for letter in word:
            node = node._get_or_create_child(letter)
            node.value = value
        return node
    
    def find_prefix(self, input: str) -> str:
        node: TrieNode = self
        for letter in input:
            if letter not in node.children:
                return None
            node = node.children[letter]
            if len(node.children) == 0:
                return node.value
        return None

class DavidSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        lines = s.split("\n")
        
        trie = TrieNode()

        digits_mapping = {
            'one': '1',
            'two': '2',
            'three': '3',
            'four': '4',
            'five': '5',
            'six': '6',
            'seven': '7',
            'eight': '8',
            'nine': '9',
        }

        for word, digit in digits_mapping.items():
            trie.insert_word(word, digit)

        for x in range(1, 10):
            trie.insert_word(str(x), str(x))

        counter = 0
        for line in lines:
            for i in range(len(line)):
                first_digit = trie.find_prefix(line[i:])
                if first_digit is not None:
                    break

            for i in range(len(line)-1, -1, -1):
                last_digit = trie.find_prefix(line[i:])
                if last_digit is not None:
                    break

            counter += int(first_digit + last_digit)

        return counter
