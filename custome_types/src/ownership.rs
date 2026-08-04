// 演示三种写法：借用 vs 拥有（String）
// cargo run --bin ownership

fn main() {
    // ============ 情况 1：借用变量（&str）—— 会跟着变 ============
    println!("=== 情况 1：借用变量（&original）—— 会跟着变 ===");
    let mut original = String::from("Peter");   // 原件，可变

    // 用一个作用域块，让借用 name 结束后，再修改 original
    {
        let name: &str = &original;             // 借用 original
        println!("改之前: original = {}, name = {}", original, name);
        // 这里 name 和 original 指向同一块内存，所以显示一样
    } // 借用 name 在这里结束（作用域结束）

    // 借用结束后，才能修改 original（Rust 规则：不能和借用同时存在）
    original.push_str(" Wong");                 // 现在可以改了
    println!("改之后: original = {}", original);
    // 说明：借用期间 name 和 original 是同一块数据；
    // 想刚才那样同时打印两者和修改，Rust 会禁止，因为那是"同时可变+不可变借用"

    println!();

    // ============ 情况 2：拥有（String::from）—— 独立不变 ============
    println!("=== 情况 2：拥有（String::from）—— 独立不变 ===");
    let mut own = String::from("Peter");         // 自己拥有一份
    let copy = own.clone();                      // 复制一份给 copy（独立）

    println!("改之前: own = {}, copy = {}", own, copy);

    own.push_str(" Wong");                       // 修改 own
    println!("改之后: own = {}, copy = {}", own, copy);
    // 注意：copy 还是 "Peter"，没变！因为 copy 有自己的一份数据

    println!();

    // ============ 情况 3：字面量借用（&str 字面量）—— 没人能改 ============
    println!("=== 情况 3：字面量借用（\"Peter\"）—— 没人能改 ===");
    let name3 = "Peter";                         // 借用只读字面量
    println!("name3 = {}", name3);
    // 字面量是程序里写死的，不可变，所以没有"原件被改"的问题
}