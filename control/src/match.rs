fn main() {
    let triple = (0, -2, 3);
    // TODO ^ 尝试为 `triple` 赋不同的值

    println!("告诉我关于 {:?} 的信息", triple);
    // match 可用于解构元组
    match triple {
        // 解构第二和第三个元素
        (0, y, z) => println!("第一个是 `0`，`y` 是 {:?}，`z` 是 {:?}", y, z),
        (1, ..)  => println!("第一个是 `1`，其余的不重要"),
        (.., 2)  => println!("最后一个是 `2`，其余的不重要"),
        (3, .., 4)  => println!("第一个是 `3`，最后一个是 `4`，其余的不重要"),
        // `..` 可用于忽略元组中的其余部分
        _      => println!("它们是什么并不重要"),
        // `_` 表示不将值绑定到变量
    }

    // 分配一个 `i32` 类型的引用。`&` 表示
    // 正在分配一个引用。
    let reference = &4;

    match reference {
        // 如果 `reference` 与 `&val` 进行模式匹配，结果
        // 就像这样的比较：
        // `&i32`
        // `&val`
        // ^ 我们可以看到，如果去掉匹配的 `&`，那么 `i32`
        // 应该被赋值给 `val`。
        &val => println!("通过解构获得的值：{:?}", val),
    }

    // 为了避免 `&`，你可以在匹配前解引用。
    match *reference {
        val => println!("通过解引用获得的值：{:?}", val),
    }

    // 如果你一开始没有引用怎么办？`reference` 是一个 `&`
    // 因为右侧已经是一个引用。这不是
    // 一个引用，因为右侧不是引用。
    let _not_a_reference = 3;

    // Rust 提供 `ref` 正是为了这个目的。它修改了
    // 赋值，为元素创建一个引用；
    // 这个引用被赋值。
    let ref _is_a_reference = 3;

    // 相应地，通过定义两个没有引用的值，
    // 可以通过 `ref` 和 `ref mut` 获取引用。
    let value = 5;
    let mut mut_value = 6;

    // 使用 `ref` 关键字创建引用。r其实是一个指针，比C++安全，但是println会自动解析其中的地址，得到值，因此最后会是5，如果想要地址，应该是{:p}
    match value {
        ref r => println!("获得了一个值的引用：{:?}", r),
    }

    // 类似地使用 `ref mut`。
    match mut_value {
        ref mut m => {
            // 获得了一个引用。在我们能够
            // 对其进行任何添加操作之前，必须先解引用。
            *m += 10;
            println!("我们加了 10。`mut_value`：{:?}", m);
        },
    }

    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo {x: (1, 2), y: 3};

        match foo {
        Foo { x: (1, b), y } => println!("x 的第一个元素是 1，b = {}，y = {}", b, y),

        // 你可以解构结构体并重命名变量，
        // 顺序并不重要
        Foo { y: 2, x: i } => println!("y 为 2，i = {:?}", i),

        // 你也可以忽略某些变量：
        Foo { y, .. } => println!("y = {}，我们不关心 x 的值", y),
        // 这会导致错误：模式中未提及字段 `x`
        //Foo { y } => println!("y = {}", y),

        
    }
}