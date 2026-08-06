// This declaration will look for a file named `my.rs` and will
// insert its contents inside a module named `my` under this scope

// 本质上是模块之间的套用，只要是pub，或者访问域一致，则可以调用
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}
