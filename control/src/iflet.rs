use std::str::FromStr;

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("无法分割计数项对：'{s}'");
    };
    let Ok(count) = u64::from_str(count_str) else {
        panic!("无法解析整数：'{count_str}'");
    };
    (count, item)
}

fn main() {
    // 以下都是 `Option<i32>` 类型
    // Option<i32> 不是基础类型，是一个泛型枚举（enum）——它表示"可能有一个 i32，也可能没有"。
    // 因为 Rust 没有 C++ 的 null 或空指针。在 C++ 里，一个函数可能返回 nullptr，调用方忘记检查就崩溃。Rust 用 Option
    // 把"可能没有值"写进类型里，编译器逼你处理 None 的情况
    // 总的来说，Option<i32> 是一个泛型枚举，表示"可能有一个 i32 值，也可能没有"——Some(5) 是有值，None 是没值。<i32>
    // 是泛型参数，告诉编译器里面装的是 i32 类型的值。它和 i32 是不同的类型，需要 match、if let 或 .unwrap() 才能取出里面的值。
    let number = Some(7);   
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `if let` 结构的含义是：如果 `let` 能将 `number` 解构为
    // `Some(i)`，则执行代码块（`{}`）。
    if let Some(i) = number {
        println!("匹配到 {:?}！", i);
    }

    // 如果需要指定匹配失败的情况，可以使用 else：
    if let Some(i) = letter {
        println!("匹配到 {:?}！", i);
    } else {
        // 解构失败。转到失败处理的情况。
        println!("没有匹配到数字。那就用一个字母吧！");
    }

    // 提供一个修改后的失败条件。
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("匹配到 {:?}！", i);
    // 解构失败。评估 `else if` 条件，看是否应该执行替代的失败分支：
    } else if i_like_letters {
        println!("没有匹配到数字。那就用一个字母吧！");
    } else {
        // 条件判断为假。这个分支是默认情况：
        println!("我不喜欢字母。那就用个表情符号吧 :)！");
    }

    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
}