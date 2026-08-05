fn main() {
    // 创建 `Option<i32>` 类型的 `optional`
    let mut optional = Some(0);

    // 这段代码的含义是：当 `let` 将 `optional` 解构为 `Some(i)` 时，
    // 执行代码块 `{}`，否则 `break`。
    while let Some(i) = optional {
        if i > 9 {
            println!("大于 9，退出！");
            optional = None;
        } else {
            println!("`i` 是 `{:?}`。再试一次。", i);
            optional = Some(i + 1);
        }
        // ^ 减少了代码缩进右移，无需显式处理失败情况
    }
    // ^ `if let` 可以有额外的 `else`/`else if` 子句，`while let` 则没有。
    // 整体是一个表达式，如果有则执行，没有则不执行
    
}