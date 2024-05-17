# RN.rs
## CLI tool for renaming files by regular expression
### Usage:
```bash
cd rn && cargo build --release
cd rn/target
rn --help
```
Or move it directily to binary folder in $PATH

### Help:
```
PROGRAM (Rust compiled .rs) FOR GRIDSEARCHING PDF FILENAMES AND RENAME ACCORDINGLY TO A SPECIFIC PATTERN

Usage: rn [OPTIONS] --folder <FOLDER>

Options:
  -f, --folder <FOLDER>  Folder to search (ex: C:\Users\user\Documents\...)
      --regex <REGEX>    Regex pattern for renaming (ex: "^a-zA-Z0-9_-")
      --log              Enable log writing
  -h, --help             Print help
  -V, --version          Print version
```