fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("修改前：{}", mutable_binding);

    // 正确
    mutable_binding += 1;

    println!("修改后：{}", mutable_binding);

    // 错误！不能给不可变变量赋新值
    // _immutable_binding += 1;


    // 这个绑定存在于 main 函数中
    let long_lived_binding = 1;

    // 这是一个代码块，它的作用域比 main 函数小
    {
        // 这个绑定只存在于此代码块中
        let short_lived_binding = 2;

        println!("内部 short：{}", short_lived_binding);
    }
    // 代码块结束

    // 错误！`short_lived_binding` 在此作用域中不存在
    // println!("外部 short：{}", short_lived_binding);
    // 修复：^ 注释掉此行

    println!("外部 long：{}", long_lived_binding);

    println!("outer long: {}", long_lived_binding);

    // 声明一个变量绑定
    let a_binding;

    {
        let x = 2;

        // 初始化绑定
        a_binding = x * x;
    }

    println!("绑定：{}", a_binding);

    let another_binding;

    // 错误！使用未初始化的绑定
    // println!("另一个绑定：{}", another_binding);
    // 修复：^ 注释掉此行

    another_binding = 1;

    println!("另一个绑定：{}", another_binding);

    let mut _mutable_integer = 7i32;

    {
        // 通过不可变的 `_mutable_integer` 进行遮蔽
        let _mutable_integer = _mutable_integer;

        // 错误！`_mutable_integer` 在此作用域中被冻结
        // _mutable_integer = 50;
        // 修复：^ 注释掉此行

        // `_mutable_integer` 离开作用域
    }

    // 正确！`_mutable_integer` 在此作用域中未被冻结
    _mutable_integer = 3;
}
