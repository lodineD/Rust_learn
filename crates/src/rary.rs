
// rustc --crate-type=lib rary.rs
// 可以使用上述命令新建库
pub fn public_function() {
    println!("调用了 rary 的 `public_function()`");
}

fn private_function() {
    println!("调用了 rary 的 `private_function()`");
}

pub fn indirect_access() {
    print!("调用了 rary 的 `indirect_access()`，它\n> ");

    private_function();
}