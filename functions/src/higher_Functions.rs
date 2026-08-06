fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Find the sum of all the numbers with odd squares under 1000");
    let upper = 1000;

    // Imperative approach
    // Declare accumulator variable
    let mut acc = 0;
    // Iterate: 0, 1, 2, ... to infinity
    for n in 0.. {
        // Square the number
        let n_squared = n * n;

        if n_squared >= upper {
            // Break loop if exceeded the upper limit
            break;
        } else if is_odd(n_squared) {
            // Accumulate value, if it's odd
            acc += n;
        }
    }
    println!("imperative style: {}", acc);

    // Functional approach
    // 高阶函数 = 接收闭包/函数作为参数。这段代码里 take_while、filter、sum
    //  take_while — 从前面取，直到条件不满足
    //  filter — 只保留满足条件的
    let sum: u32 =
        (0..).take_while(|&n| n * n < upper) // Below upper limit
             .filter(|&n| is_odd(n * n))     // That are odd
             .sum();                         // Sum them
    println!("functional style: {}", sum);
}