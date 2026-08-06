// 这个函数只有在目标操作系统是 linux 时才会被编译
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("你正在运行 Linux！");
}

// 而这个函数只有在目标操作系统**不是** Linux 时才会被编译
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("你**不是**在运行 Linux！");
}

fn main() {
    are_you_on_linux();

    println!("你确定吗？");
    if cfg!(target_os = "linux") {
        println!("是的，这绝对是 Linux！");
    } else {
        println!("是的，这绝对**不是** Linux！");
    }
}