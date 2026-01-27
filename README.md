# Code line count (clc)

Counts the total non-empty lines of code in files matching given categories
or file extensions, recursively.

## Install

```sh
cargo install code-line-count
```

## Usage

```text
clc [OPTION | CATEGORY | .EXT]...
```

Options, categories, and extensions may be mixed in any order.

## Options

| Flag            | Description                          |
| --------------- | ------------------------------------ |
| `--help`        | display help text and exit           |
| `-v, --version` | display version and exit             |
| `-dN`           | set maximum search depth to N        |
| `-g, --git`     | respect .gitignore files             |
| `-h, --hidden`  | include hidden files and directories |

## Categories

| Category                 | Extensions                                                                  |
| ------------------------ | --------------------------------------------------------------------------- |
| `rust/rs`                | rs, rlib                                                                    |
| `haskell/hs`             | hs, lhs                                                                     |
| `kotlin/kt`              | kt, kts, kexe, klib                                                         |
| `csharp/c#/cdim`         | cs, csx                                                                     |
| `java`                   | java, class, jmod, war                                                      |
| `zig`                    | zig, zir, zigr, zon                                                         |
| `c`                      | c, h                                                                        |
| `golang/go`              | go                                                                          |
| `cplusplus/c++/cpp/hell` | c, C, cc, cpp, cxx, c++, h, H, hh, hpp, hxx, h++, cppm, ixx                 |
| `web/webdev`             | js, jsx, ts, tsx, mjs, cjs, css, scss, sass, less, styl, vue, svelte, astro |
| `react`                  | tsx, jsx                                                                    |
| `typescript`             | tsx, ts                                                                     |
| `javascript`             | jsx, js                                                                     |
| `php`                    | php, phar, phtml, pht, phps                                                 |
| `ruby`                   | rb, ru                                                                      |
| `elixir/ex`              | ex, exs                                                                     |
| `python/py`              | py                                                                          |
| `shell`                  | sh, bash, zsh, fish                                                         |
| `styles/css`             | css, scss, sass, less                                                       |
| `config/cfg`             | toml, yaml, yml, json, cfg                                                  |
| `markup`                 | html, md                                                                    |

## Examples

```sh
clc .rs .hs           # count Rust and Haskell files
clc -g web .py        # count 'web' category and Python files, respecting .gitignore
clc -h -d3 .toml      # include hidden files, max depth 3
```
