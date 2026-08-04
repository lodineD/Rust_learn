// Globals are declared outside all other scopes.
/*
 1. const —— 编译期常量，用一次复制一次

  const MAX_POINTS: u32 = 100_000;

  特点：
  - 编译期就确定，值写死在代码里。
  - 没有固定内存地址——每次用到 MAX_POINTS，编译器直接把 100_000 复制过去。
  - 它不是"变量"，是"常量名"。程序运行时不占内存，是编译时展开的。

  类比：const 就像"菜谱上的配料表"——每道菜用到盐的时候，直接看配料表撒一份，不会有一包共享的盐。

  2. static —— 全局单例，固定内存地址

  static APP_NAME: &str = "MyApp";

  特点：
  - 程序里只有一个实体，固定内存地址，整个程序运行期间都存在（'static 生命周期）。
  - 可变的 static mut 访问是不安全的（需要 unsafe），因为多线程共享同一块内存有数据竞争风险。
  - 只读的 static 可以安全共享，但多线程下要求类型是 Sync。

  类比：static 就像"公司唯一的招牌"——全公司就这一个，挂在门口，谁都能看到（读），但要改它就得小心别人正看着。
*/
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line
}