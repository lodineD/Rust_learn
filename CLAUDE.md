# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目定位

这是一个 Rust 学习项目（Rust by Example 风格），不是产品代码。目标是循序渐进地学 Rust，最终能独立做项目。所有 `.rs` 文件都是**独立的、可单独运行的学习示例**，彼此没有调用关系。

## 构建与运行

- 根目录下有 **6 个相互独立的 cargo package**，每个对应 Rust by Example 的一个章节（目录名即章节名）：`hello_world`（fmt 格式化）、`primitives`（基础类型）、`custome_types`（自定义类型）、`variable_bindings`（变量绑定）、`types`（类型转换）、`conversion`（From/Into 等转换 trait）。全部是 edition 2024、无外部依赖、无测试、无 lint 配置。
- 构建/运行命令都**需要在对应 package 目录内**执行：
  - `cargo run` —— 用该 package 的 `Cargo.toml` 里的 `default-run` 运行默认 bin（每个 package 的 `default-run` 随学习进度变动，通常是当前在学的示例）。
  - `cargo run --bin <名字>` —— 运行指定 bin。
  - `cargo build` —— 编译整个 package（会检查所有 bin 能否编译通过）。
- **关键：每个 package 有多个二进制入口（bin）**，每个 `src/*.rs` 对应一个。新增一个可运行示例时，**除了创建文件，还要在对应 package 的 `Cargo.toml` 的 `[[bin]]` 段注册它**，否则 `cargo run --bin` 无法识别。
- 每个 package 的 bin 命名基本对应源文件名，但保留 Rust by Example 原样的大小写（如 `Enums_Clike.rs`、`Mutability.rs`、`String.rs`）。

## 架构要点

- 每个 bin 是自包含的 `fn main()`，通过 `use std::fmt;`、`use std::convert::From;` 等直接使用标准库，无共享模块。
- 示例覆盖 Rust 基础概念：`fmt`（Display/Debug）、`impl` 与 trait、`enum` 与 `match`、`Option`/`Result`、所有权与借用、`From`/`Into`/`FromStr`/`TryFrom` 等类型转换 trait。
- **帮助用户时，教学式解释比直接改代码更有价值**——用户是初学者，会追问概念（如 `impl`、`trait`、`enum`、`mut`、`String` vs `&str`、`Option` vs `Result`）。用户中文交流，解释用中文。
- 用户会在学习示例里自己加注释记录理解；这些注释可能有语病或概念不精确，除非影响编译否则不主动改动。用户学完一章后会把新增示例提交到 git（提交信息用中文，格式如 `学习：新增 xxx 示例`）。
