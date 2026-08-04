use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Circle {
    radius: i32,
}

// FromStr是.parse中的trait，如果你现在在非字符串中使用，则自定义即可，下面是对一个结构体所配置的协议
impl FromStr for Circle {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(num) => Ok(Circle { radius: num }),
            Err(e) => Err(e),
        }
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "半径为 {} 的圆", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle);
    println!("{:?}", circle.to_string());

    // "5".parse()默认是i32
    // 返回：Result<i32, ParseIntError>
    //        ↑        ↑
    //        成功时  失败时
    //        装的是   装的是
    //        数字5    错误信息

    // .parse() 的返回类型是 Result<i32, ParseIntError>——一个"结果容器"：
    // - 成功：Ok(5)——里面装着数字 5。
    // - 失败：Err(...)——里面装着错误信息。
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("总和：{:?}", sum);
}
