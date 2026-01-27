// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Copyright (c) 2026 c1ph3rC4t

//! Code line count (clc)
//!
//! Counts the total non-empty lines of code in files matching given categories
//! or file extensions, recursively.
//!
//! Run `clc --help` for usage and options.

#[doc(hidden)]
mod cats;
#[doc(hidden)]
mod partition_n;

use clap::Parser;
use ignore::{WalkBuilder, WalkState::Continue};
use memchr::memchr_iter;
use partition_n::PartitionN;
use regex::bytes::Regex;
use std::fmt::Write;
use std::{fs, path::PathBuf, process::exit, sync::mpsc};
use thiserror::Error;

define_categories! {
    Rust => {
        names: ["rust", "rs"],
        extensions: ["rs", "rlib"],
    },
    Haskell => {
        names: ["haskell", "hs"],
        extensions: ["hs", "lhs"],
    },
    Kotlin => {
        names: ["kotlin", "kt"],
        extensions: ["kt", "kts", "kexe", "klib"],
    },
    CSharp => {
        names: ["csharp", "c#", "cdim"],
        extensions: ["cs", "csx"],
    },
    Java => {
        names: ["java"],
        extensions: ["java", "class", "jmod", "war"],
    },
    Zig => {
        names: ["zig"],
        extensions: ["zig", "zir", "zigr", "zon"],
    },
    C => {
        names: ["c"],
        extensions: ["c", "h"],
    },
    GoLang => {
        names: ["golang", "go"],
        extensions: ["go"],
    },
    Cpp => {
        names: ["cplusplus", "c++", "cpp", "hell"],
        extensions: ["c", "C", "cc", "cpp", "cxx", "c++", "h", "H", "hh", "hpp", "hxx", "h++", "cppm", "ixx"],
    },
    Web => {
        names: ["web", "webdev"],
        extensions: ["js", "jsx", "ts", "tsx", "mjs", "cjs", "css", "scss", "sass", "less", "styl", "vue", "svelte", "astro"],
    },
    React => {
        names: ["react"],
        extensions: ["tsx", "jsx"],
    },
    TypeScript => {
        names: ["typescript"],
        extensions: ["tsx", "ts"],
    },
    JavaScript => {
        names: ["javascript"],
        extensions: ["jsx", "js"],
    },
    PHP => {
        names: ["php"],
        extensions: ["php", "phar", "phtml", "pht", "phps"],
    },
    Ruby => {
        names: ["ruby"],
        extensions: ["rb", "ru"],
    },
    Elixir => {
        names: ["elixir", "ex"],
        extensions: ["ex", "exs"],
    },
    Python => {
        names: ["python", "py"],
        extensions: ["py"],
    },
    Shell => {
        names: ["shell"],
        extensions: ["sh", "bash", "zsh", "fish"],
    },
    Styles => {
        names: ["styles", "css"],
        extensions: ["css", "scss", "sass", "less"],
    },
    Config => {
        names: ["config", "cfg"],
        extensions: ["toml", "yaml", "yml", "json", "cfg"],
    },
    Markup => {
        names: ["markup"],
        extensions: ["html", "md"],
    },
}

#[doc(hidden)]
#[derive(Parser)]
#[command(
    disable_help_flag = true,
    disable_help_subcommand = true,
    disable_version_flag = true
)]
struct Args {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

/// Errors that can occur during CLC operations.
#[derive(Debug, Error)]
pub enum CLCError {
    /// Regex compilation or matching error.
    ///
    /// Derived from [`regex::Error`]
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),

    /// I/O operation error.
    ///
    /// Derived from [`std::io::Error`]
    #[error("I/O error: {0}")]
    IOError(#[from] std::io::Error),
}

/// Generates a help string for clc
#[must_use]
pub fn gen_help() -> String {
    let mut cat_strings = vec![];
    let mut ext_strings = vec![];
    let mut cat_list = "Categories:".to_string();
    let mut longest_cat_string = 0;

    for cat in CategoryID::all_ids() {
        let mut cat_string = String::new();
        let mut ext_string = String::new();

        for (idx, name) in CategoryID::names(*cat).iter().enumerate() {
            match idx {
                0 => cat_string += name,
                _ => {
                    let _ = write!(cat_string, "/{name}");
                }
            }
        }

        for (idx, ext) in CategoryID::extensions(*cat).iter().enumerate() {
            match idx {
                0 => ext_string += ext,
                _ => {
                    let _ = write!(ext_string, ", {ext}");
                }
            }
        }

        if cat_string.len() > longest_cat_string {
            longest_cat_string = cat_string.len();
        }
        cat_strings.push(cat_string);
        ext_strings.push(ext_string);
    }

    for (idx, cat_string) in cat_strings.iter().enumerate() {
        let _ = write!(
            cat_list,
            "\n  {}{} | {}",
            cat_string,
            " ".repeat(longest_cat_string - cat_string.len()),
            ext_strings[idx],
        );
    }

    format!(
        "Usage: clc [OPTION | CATEGORY | .EXT]...
Count non-empty lines of code in files matching CATEGORY or .EXT, recursively.
Example: clc -g .py web -d2 .rs

Arguments may be given in any order:
  starting with '-'         option
  starting with '.'         file extension
  otherwise                 category

Options:
      --help                display this help text and exit
  -v, --version             display version and exit
  -dN                       set maximum search depth to N
  -g, --git                 respect .gitignore files
  -h, --hidden              include hidden files and directories

{cat_list}"
    )
}

/// Counts non-empty lines of code.
///
/// Uses [`WalkBuilder`] from the [`ignore`] crate to create
/// a multi-threaded code line counter.
///
/// # Errors
///
/// Returns [`CLCError`] if regex compilation fails or if file I/O
/// operations fail (e.g., permission denied, unable to read file contents).
///
/// # Example
///
/// ```
/// let lines = count_lines(
///     PathBuf::from("./"),
///     &["rs", "hs"],
///     true,
///     true,
///     None,
/// )?;
/// ```
pub fn count_lines(
    path: PathBuf,
    exts: &[&str],
    hidden: bool,
    respect_git_ignore: bool,
    maxdepth: Option<usize>,
) -> Result<u128, CLCError> {
    let re = &Regex::new(r"\n\s+")?;
    let (tx, rx) = mpsc::channel();

    WalkBuilder::new(path)
        .hidden(!hidden)
        .ignore(false)
        .git_ignore(respect_git_ignore)
        .max_depth(maxdepth)
        .build_parallel()
        .run(|| {
            let tx = tx.clone();
            Box::new(move |entry| {
                let Ok(entry) = entry else { return Continue };

                let path = entry.path();

                if entry.file_type().is_none_or(|ft| !ft.is_file()) {
                    return Continue;
                }

                let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

                if !exts.contains(&ext) {
                    return Continue;
                }

                let result = fs::read(path).map_or_else(
                    |_| unreachable!(),
                    |bytes| {
                        memchr_iter(b'\n', &re.replace_all(&bytes, b"\n")).count()
                            + usize::from(!bytes.ends_with(b"\n"))
                    },
                );

                tx.send(result).ok();

                Continue
            })
        });

    drop(tx);
    Ok(rx.iter().map(|n| n as u128).sum())
}

#[doc(hidden)]
fn main() -> Result<(), CLCError> {
    let mut exts: Vec<&str> = vec![];
    let mut hidden = false;
    let mut respect_git_ignore = false;
    let mut maxdepth = None;
    let depth_re = Regex::new(r"^\-d[1-9][0-9]+$")?;
    let args = Args::parse().args;

    let [flags, extargs, cats]: [Vec<&str>; 3] =
        args.iter().map(String::as_str).partition_n(|arg| {
            if arg.starts_with('-') {
                0
            } else if arg.starts_with('.') {
                1
            } else {
                2
            }
        });

    let flags = flags.clone();

    for flag in flags {
        match flag.as_bytes() {
            b"--help" => {
                println!("{}", gen_help());
                exit(0)
            }
            b"-v" | b"--version" => {
                println!("clc {}", env!("CARGO_PKG_VERSION"));
                exit(0)
            }
            b"-h" | b"--hidden" => hidden = true,
            b"-g" | b"--git" => respect_git_ignore = true,
            flag_bytes if depth_re.is_match(flag_bytes) => {
                maxdepth = flag[2..].parse().ok();
            }
            _ => {
                println!(
                    "clc: flag \"{flag}\" not found\nTry 'clc --help' for more information on how to use clc."
                );
                exit(0)
            }
        }
    }

    for cat_name in cats {
        if let Some(cat_id) = CategoryID::from_name(cat_name) {
            exts.extend(cat_id.extensions());
        } else {
            println!(
                "clc: category {cat_name} not found\nTry 'clc --help' for more information on how to use clc."
            );
            exit(0)
        }
    }

    for ext in extargs {
        exts.push(ext.get(1..).unwrap_or(""));
    }

    if exts.is_empty() {
        println!("clc: missing operand\nTry 'clc --help' for more information on how to use clc.");
        exit(0)
    }

    let lines = count_lines(
        PathBuf::from("./"),
        &exts,
        hidden,
        respect_git_ignore,
        maxdepth,
    )?;

    println!("{lines}");

    Ok(())
}
