# Rust_learn

> 我必须逼自己每天学一点，一个月后可以着手做一点小的项目或功能，3个月后做大项目；如果我没有做到，那我就一个月不买游戏(੭ ᐕ)੭？

## Rust by example（以身作则的锈蚀）

| 日期 | 学习进度 |
|------|---------|
| **2026-08-04** | 第一天：hello_world、primitives、custome_types、variable_bindings、types、conversion |
| **2026-08-05** | 第二天：control、functions（基础） |
| **2026-08-06** | 第三天：functions（闭包与高阶函数）、modules（模块与可见性）、crates（库与 crate）、generics（泛型） |

### 📅 第一天（2026-08-04）

- **hello_world**（fmt 格式化：Display/Debug、ANSI 颜色）
- **primitives**（基础类型：元组、数组与切片、运算符）
- **custome_types**（自定义类型：结构体、枚举、常量 const/static、所有权）
- **variable_bindings**（变量绑定：可变性 mut、作用域、变量遮蔽）
- **types**（类型转换：as 转换、字面量、类型推断）
- **conversion**（类型转换 trait：From/Into、FromStr/parse、TryFrom/TryInto）

### 📅 第二天（2026-08-05）

- **control**（流程控制：loop/for/match/守卫/if-let/while-let）
- **functions**（函数与方法：fn、方法 &self/&mut self）

### 📅 第三天（2026-08-06）

- **functions**（闭包、高阶函数、发散函数）
- **modules**（模块 `mod`、可见性 `pub`、`self`/`super`/`crate`、文件拆分、`use` 导入）
- **crates**（crate 与库：创建库 `rary`、引用库模块、`mod` 声明）
- **generics**（泛型：泛型函数、泛型 trait）

#### 闭包总结

> 闭包适合临时调用的小逻辑，复杂逻辑应使用普通函数。

- **定义**：匿名函数，可捕获外层局部变量，用 `|参数| 函数体` 定义。
- **三种 trait**：
  - `Fn`：只读，可调用多次。
  - `FnMut`：可修改捕获变量，可调用多次。
  - `FnOnce`：消耗所有权，只能调用一次（如 `mem::drop`）。
- **输入输出**：输入用 `<F> + where F: Fn()`；输出用 `-> impl Fn()`，返回闭包需加 `move` 防止悬垂引用。
- **用途**：迭代器 `map`/`filter`/`any`、自定义排序 `sort_by`、`Option`/`Result` 链式调用。

#### `iter()` vs `into_iter()`

| 类型 | `iter()` | `into_iter()` | 之后还能用吗 |
|------|----------|---------------|:---:|
| `Vec<i32>` | 借用，产生 `&i32` | 拿走所有权，产生 `i32` | ❌ |
| `[i32; 3]`（数组） | 借用，产生 `&i32` | 不拿所有权，产生 `i32` | ✅ |

数组是 `Copy` 类型，`into_iter()` 不移动所有权；`Vec` 不是 `Copy`，`into_iter()` 会拿走所有权。

#### 模块可见性

| 修饰符 | 可见范围 | 说明 |
|--------|---------|------|
| （默认） | 当前模块内 | 私有，外部不可访问 |
| `pub(self)` | 当前模块内 | 与默认相同 |
| `pub(super)` | 父模块 | 只在上一层模块可见 |
| `pub(in crate::xxx)` | 指定路径 | 只在指定的祖先模块内可见 |
| `pub(crate)` | 当前 crate | 整个 bin/lib 内可见 |
| `pub` | 全部 | 同一 crate 及外部 crate（对 lib） |

私有父模块会限制内部子项的可见性，即使子项标记了 `pub`。不同 bin 之间互相隔离，`pub` 也不能跨 bin 访问。

---

> 纸上得来终觉浅，绝知此事要躬行。

三天时间，把 Rust by Example 从头刷到尾，`hello_world` 到 `generics`，11 个 cargo 项目，基本概念都过了一遍。fmt、枚举、trait、所有权、闭包、模块、泛型、生命周期……每个都"学过"了。

但说实话，trait 和泛型放在一起怎么用？生命周期标注到底什么时候该写、什么时候该省略？闭包的三种 trait 怎么选？这些光靠看 demo 是学不会的。demo 只会告诉你"这样写是对的"，不会告诉你"为什么我那样写会爆 20 个编译错误"。真正的理解，是跟编译器吵出来的——每一次 `error[E0502]`、每一次 `cannot borrow as mutable`、每一次 `does not live long enough`，都是在帮你重新理解"所有权和借用"那几个字到底是什么意思。

所以这不是结束，是新的开始。接下来不再逐章刷文档，而是尝试写一些小的东西——也许是一个命令行小工具，也许是一个简单的数据结构，也许是造几个轮子，在实战中与编译器 battle，把"学过"变成"会用"。

**Rust 不会放过任何偷懒，但也从不辜负认真。**
