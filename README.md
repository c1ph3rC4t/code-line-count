# CLC(code-line-count)

Fast line counter for code files.

## Installation

```sh
cargo install --path .
```

## Usage

```
clc [FLAG OR CATEGORY OR FILE EXTENSION]...
```

Count lines in Rust files:
```sh
clc rust
```

Count lines in multiple categories:
```sh
clc rust python web
```

Count by file extension:
```sh
clc .rs .py
```

### Flags

- `--help` - display help text
- `-v, --version` - display version
- `-h, --hidden` - include hidden files
- `-g, --git` - respect .gitignore
- `-d#` - set search depth

## License

MPL-2.0
