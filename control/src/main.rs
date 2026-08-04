/*

宏是一种**"写代码的代码"——它在编译时生成代码**。你可以理解成"高级模板/快捷方式"。

对比：
- 函数：接收参数，运行，返回结果。
- 宏：在编译阶段展开成一堆代码，然后再编译。

println! 为什么必须是宏？

因为 println! 有两个函数做不到的能力：

1. 接受任意数量的参数

println!("{}", 1);          // 1 个参数
println!("{} {}", 1, 2);    // 2 个参数
println!("{} {} {}", 1, 2, 3);  // 3 个参数

普通函数必须提前声明固定数量的参数（比如 fn f(a, b, c) 只能接 3 个）。但 println! 能接受任意多参数——这是函数做不到的，只有宏能。

2. 检查格式化字符串（编译期）

println!("{} {}", 1);   // 编译错误！{} 有两个，但只给了 1 个值

宏可以在编译时检查 {} 占位符和参数数量是否匹配。函数做不到这种编译期检查。

一个类比

把 println! 想成**"打印公式"**：

- 普通函数 = 一台固定规格的机器，只能处理规定数量的输入。
- 宏 = 一个"模板生成器"，你告诉它格式，它在编译时自动生成合适的代码。

println!("{} {}", a, b) 这个宏，在编译时会被展开成类似这样的代码（简化）：

// println! 展开后大概是这样的（简化）
std::io::_print(format_args!("{} {}", a, b));

你写一行宏，编译器帮你展开成好几行代码。

常见宏清单（都有 !）

你这一路已经见过很多宏了：

println!()      // 打印到屏幕
print!()        // 打印（不带换行）
format!()       // 生成字符串（format.rs 里用过）
write!()        // 写入 Formatter（Display 里用过）
vec![]          // 创建向量
assert_eq!()    // 断言相等（arrays.rs 里用过）
write!()
 */


fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This expression must return an `i32` as well.
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon.
        };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.
    // 这是宏，不是函数
    println!("{} -> {}", n, big_n);

        let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
}