[![CI status](https://github.com/LimeEng/aoc/actions/workflows/ci.yaml/badge.svg)](https://github.com/LimeEng/aoc/actions/workflows/ci.yaml)

<img src="https://cdn.github.emileng.se/repo/aoc/festive_ferris.svg" width="150" alt="Festive Ferris" align="center">

# Advent of Code

These are my solutions to Advent of Code.

## Usage

```sh
# The AOC_KEY environment variable must be set
export AOC_KEY="secret-key"
# The AOC_SESSION environment variable must be set if API-related functionality is used.
export AOC_SESSION="secret-session"

# Run solutions interactively or by specifying year/day/part
aoc solve [year] [day] [part]

# Decrypt puzzle data to edit test cases or metadata
aoc decrypt

# Edit files in puzzles/...

# Re-encrypt for use with solve and tests
aoc encrypt

# Download and encrypt puzzle prompts and inputs.
# Downloads are cached locally; the remote server is contacted only if needed.
aoc download <year> <day> <part>

# Run the full test suite (executes solutions against all test-cases and inputs)
cargo test

# Run benchmarks (uses Criterion to measure solution performance)
cargo bench

# Build and open documentation locally
# Puzzle descriptions are included in the generated documentation
cargo doc --open
```

## Encrypted puzzle data

[This directory](puzzles.enc/) contains encrypted puzzle data including inputs, expected outputs, and metadata. Advent of Code asks that participants not include puzzle text or inputs in code repositories, so everything is stored encrypted to respect that request while keeping solutions verifiable.

Of course, anyone can simply log in and grab their own data — but the restriction did inspire a fun workaround.

## Automation

This repository follows Advent of Code's [automation guidelines](https://old.reddit.com/r/adventofcode/wiki/faqs/automation). Downloads are cached locally and always explicitly initiated by the user. If the requested content is already cached, the remote server is not contacted. All requests include descriptive [`User-Agent`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/User-Agent) and [`From`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/From) headers so site operators can reach out if needed.

The tooling itself does not enforce throttling, so be mindful and avoid rapid-fire requests to the Advent of Code servers.
