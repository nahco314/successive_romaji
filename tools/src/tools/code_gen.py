import sys

from tools.base_romajis import ROMAJIS


def main():
    basic_romajis = ROMAJIS.copy()

    basic_romajis.sort(key=lambda x: len(x[0]), reverse=True)

    res = f"pub const BASIC_ROMAJI_CHARS: [(&str, &str); {len(basic_romajis)}] = ["
    for hiragana, romaji in basic_romajis:
        res += f'("{hiragana}", "{romaji}"),'
    res += "];"

    print(res)

    print("done.", file=sys.stderr)
    print("do `cargo fmt` for the generated file.", file=sys.stderr)


if __name__ == '__main__':
    main()
