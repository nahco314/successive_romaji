# successive_romaji

Returns the input up to that point, the characters currently being input, and the romaji expected in the future, based on the target hiragana sequence and the romaji input up to the halfway point.
Used to implement a typing game.

## Usage

Just use `parse_hiragana_with_buf(hiragana, romaji)`.

## Python bindings

This crate has Python bindings. Source is in ./python/ .

See https://pypi.org/project/successive-romaji/ .

### Build and publish Python binding

```commandline
docker run --rm -v $(pwd):/io ghcr.io/pyo3/maturin build --release -i python3.12 --sdist
rye publish ./target/wheels/*  --skip-existing
```
