# successive_romaji

Returns the input up to that point, the characters currently being input, and the romaji expected in the future, based on the target hiragana sequence and the romaji input up to the halfway point.
Used to implement a typing game.

## Usage

Just use `parse_hiragana_with_buf(hiragana, romaji)`.
