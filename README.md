# Rust_learn

> 我必须逼自己每天学一点，一个月后可以着手做一点小的项目或功能，3个月后做大项目；如果我没有做到，那我就一个月不买游戏(੭ ᐕ)੭？

## Rust by example（以身作则的锈蚀）

| 日期 | 学习进度 |
|------|---------|
| **2026-08-04** | 第一天：hello_world、primitives、custome_types、variable_bindings、types、conversion |
| **2026-08-05** | 第二天：control、functions（基础） |
| **2026-08-06** | 第三天：functions（闭包与高阶函数）、modules（模块与可见性） |

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

[![Typing SVG](https://readme-typing-svg.demolab.com?font=Fira+Code&duration=2000&pause=1000&width=435&lines=%E6%AD%A3%E5%9C%A8%E8%BF%9B%E8%A1%8C%E4%B8%AD...)](https://git.io/typing-svg)
