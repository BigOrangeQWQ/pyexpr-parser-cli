# Expr Cli

Expr CLI is a command-line tool built with Rust.

This Cli base-on [Ruff](https://github.com/astral-sh/ruff) [Parser](https://github.com/astral-sh/ruff/blob/main/crates/ruff_python_parser) and [AST](https://github.com/astral-sh/ruff/blob/main/crates/ruff_python_ast)

---

一个基于 Ruff 里的 Parser 和 AST 模块构建出来的简单 CLI 程序

用于判断传入的字符串是否是 Python 里的 Expr 且输出其类型

若是传入错误的字符串，则程序会输出 `error`

