[![CI status](https://github.com/LimeEng/aoc/actions/workflows/ci.yaml/badge.svg)](https://github.com/LimeEng/aoc/actions/workflows/ci.yaml)

# Advent of Code

These are my solutions to Advent of Code.

## Usage

```sh
# The AOC_KEY environment variable must be set for encryption/decryption
export AOC_KEY="secret-key"

# Run solutions interactively or by specifying year/day/part
aoc solve [year] [day] [part]

# Decrypt puzzle data to edit test cases or metadata
aoc decrypt

# Edit files in puzzles/...

# Re-encrypt for use with solve and tests
aoc encrypt
```

## Encrypted puzzle data

[This directory](puzzles.enc/) contains encrypted puzzle data including test inputs, expected outputs, and metadata. Advent of Code asks that participants not include puzzle text or inputs in code repositories, so everything is stored encrypted to respect that request while keeping solutions verifiable.

Of course, anyone can simply log in and grab their own data — but the restriction did inspire a fun workaround.
