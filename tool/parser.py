# Base class for parsers
class Parser:
    def parse(self, s: str) -> str:
        raise NotImplementedError

    def cleanup(self) -> None:
        pass
