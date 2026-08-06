// 这个函数接受一个闭包作为参数并调用它
// <F> 表示 F 是一个"泛型类型参数"
// FnOnce()是闭包的trait，一共有三个trait，
//    trait  │           含义           │ 能调几次 │
//  ├────────┼──────────────────────────┼──────────┤
//  │ FnOnce │ 消耗所有权，调用一次       │ 1 次     │
//  ├────────┼──────────────────────────┼──────────┤
//  │ FnMut  │ 可变借用，能改捕获的变量   │ 多次     │
//  ├────────┼──────────────────────────┼──────────┤
//  │ Fn     │ 不可变借用，只读           │ 多次     │
//  └────────┴──────────────────────────┴──────────┘

fn apply<F>(f: F) where
    // 这个闭包不接受输入也不返回任何值
    F: FnOnce() {
    // ^ TODO：试着将其改为 `Fn` 或 `FnMut`
    // 不能改成另外两个trait
    f();
}

// 这个函数接受一个闭包并返回 `i32`
// f是闭包，是函数，返回时使用函数的形式，即f（3），而不是f=3
// 是where F: Fn(i32) -> i32约束的，让编译器知道，这个<F>范式应该传入闭包
fn apply_to_3<F>(f: F) -> i32 where
    // 这个闭包接受一个 `i32` 并返回一个 `i32`
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // 一个非复制类型
    // `to_owned` 从借用的数据创建拥有所有权的数据
    let mut farewell = "goodbye".to_owned();

    // 捕获两个变量：通过引用捕获 `greeting`，
    // 通过值捕获 `farewell`
    let diary = || {
        // `greeting` 是通过引用捕获的：需要 `Fn`
        println!("我说{}。", greeting);

        // 修改强制 `farewell` 通过可变引用捕获
        // 现在需要 `FnMut`
        farewell.push_str("！！！");
        println!("然后我喊{}。", farewell);
        println!("现在我可以睡觉了。呼呼");

        // 手动调用 drop 强制 `farewell` 通过值捕获
        // 现在需要 `FnOnce`
        // 这里会将farewell手动销毁，因此如果apply调用后，使用Fn或者FnMut,会报错，只能调用一次，farewell用完即弃
        mem::drop(farewell);
    };

    // 调用应用闭包的函数
    apply(diary);

    // `double` 满足 `apply_to_3` 的 trait 约束
    let double = |x| 2 * x;

    println!("3 的两倍是：{}", apply_to_3(double));
}