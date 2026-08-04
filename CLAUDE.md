# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目定位

这是一个 Rust 学习项目（Rust by Example 风格），不是产品代码。目标是循序渐进地学 Rust，最终能独立做项目。所有 `.rs` 文件都是**独立的、可单独运行的学习示例**，彼此没有调用关系。

## 构建与运行

- 项目根目录是 `hello_world/`（这是一个单 package，`name = "hello_world"`，edition 2024，无外部依赖）。
- 唯一的构建/运行命令：`cargo run`（或 `cargo build`）。没有测试，也没有 lint 配置。
- **关键：这个 package 有多个二进制入口（bin）**，每个 `src/*.rs` 对应一个：
  - `main.rs` → `hello_world`
  - `debug.rs` → `debug`
  - `debug2.rs` → `debug2`
  - `display.rs` → `display`
  - `display2.rs` → `display2`
- 运行方式：
  - `cargo run` —— 用 `Cargo.toml` 里的 `default-run`（当前是 `display2`）运行默认 bin。
  - `cargo run --bin <名字>` —— 运行指定 bin，例如 `cargo run --bin display`。
- 新增一个可运行示例（.rs 文件）时，**除了创建文件，还要在 `Cargo.toml` 的 `[[bin]]` 段注册它**，否则 `cargo run --bin` 无法识别。

## 架构要点

- 每个 bin 是自包含的 `fn main()`，通过 `use std::fmt;` 等直接使用标准库，无共享模块。
- 多个示例覆盖了 `fmt`（Display/Debug 实现）、`match`、`enum`、结构体、格式化参数等 Rust 基础概念，是学习参考资料。
- 帮助用户时，教学式解释比直接改代码更有价值——用户是初学者，会追问概念（如 `impl`、`trait`、`enum`、分号/表达式）。