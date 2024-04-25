from typing import Optional, Literal, TypeAlias

Romaji: TypeAlias = tuple[str, str]


def try_read(hiragana: str, romaji: str) -> Optional[tuple[str, str, Romaji]]:
    ...


def parse_hiragana(hiragana: str) -> list[Romaji]:
    ...


def parse_hiragana_with_buf(hiragana: str, romaji: str) -> tuple[Literal["Writing", "Completed"], list[Romaji], tuple[str, str, str], list[Romaji]]:
    ...
