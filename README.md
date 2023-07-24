# TSV Parse

Quick and dirty regex TSV parser built on top of [Rayon](https://github.com/rayon-rs/rayon) for fast parsing. As this is a quick and dirty script it will not be published to [crates.io](https://crates.io/).

### Usage

```
Parse a tsv file with provided regex pattern

Usage: tsv_parse <PATH> <PATTERN>

Arguments:
  <PATH>     Path to file
  <PATTERN>  Regex pattern to search

Options:
  -h, --help     Print help
  -V, --version  Print version
```
