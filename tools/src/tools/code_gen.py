import sys

from tools.base_romajis import ROMAJIS


def main():
    all_romajis = ROMAJIS.copy()

    for hiragana, romaji in ROMAJIS:
        if hiragana not in ("ん", "ー"):
            all_romajis.append((f"ん{hiragana}", f"n{romaji}"))

    for hiragana, romaji in ROMAJIS:
        if hiragana not in ("ー",) and romaji[0] not in "aiueon":
            all_romajis.append((f"っ{hiragana}", f"{romaji[0]}{romaji}"))

    all_romajis.sort(key=lambda x: len(x[0]), reverse=True)

    res = "// this file is generated by tools/code_gen.py\n"
    res += f"pub const ROMAJI_CHARS: [(&str, &str); {len(all_romajis)}] = ["
    for hiragana, romaji in all_romajis:
        res += f'("{hiragana}", "{romaji}"),'
    res += "];"

    print(res)

    print("done.", file=sys.stderr)
    print("do `cargo fmt` for the generated file.", file=sys.stderr)


if __name__ == '__main__':
    main()
