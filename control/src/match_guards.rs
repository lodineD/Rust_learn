#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn main() {
    let temperature = Temperature::Celsius(35);
    // ^ TODO try different values for `temperature`

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        // The `if condition` part ^ is a guard
        Temperature::Celsius(t) => println!("{}C is equal to or below 30 Celsius", t),

        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is equal to or below 86 Fahrenheit", t),
    }

    let number: u8 = 4;
    // 如果使用守卫模式，需要包含所有内容，否则会有问题
    match number {
        i if i == 0 => println!("零"),
        i if i > 0 => println!("大于零"),
        _ => unreachable!("不应该发生。"),
        // TODO ^ 取消注释以修复编译错误
    }
}