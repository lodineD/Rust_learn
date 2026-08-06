// `F` must implement `Fn` for a closure which takes no
// inputs and returns nothing - exactly what is required
// for `print`.
// 不是说这个例子里闭包是匿名的，其实所有闭包都是匿名的，闭包的类型是编译器自己生成的，因此，它需要泛型参数
fn apply<F>(f: F) where
    F: Fn() {
    f();
}

// 闭包可以所为输出，也可以作为输入函数
fn call_me<F: Fn()>(f: F) {
    f();
}

// 定义一个满足 `Fn` 约束的包装函数
fn function() {
    println!("我是函数！");
}

fn main() {
    let x = 7;

    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`.
    let print = || println!("{}", x);

    apply(print);

    // 定义一个满足 `Fn` 约束的闭包
    let closure = || println!("我是闭包！");

    call_me(closure);
    call_me(function);
}