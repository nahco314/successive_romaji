from successive_romaji._lowlevel import try_read, parse_hiragana, parse_hiragana_with_buf
__all__ = ["try_read", "parse_hiragana", "parse_hiragana_with_buf"]

if __name__ == '__main__':
    print(parse_hiragana_with_buf("あああ", "a"))
