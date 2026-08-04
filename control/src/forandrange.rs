fn main() {
    // `n` 在每次迭代中将取值：1, 2, ..., 100
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // `n` 在每次迭代中将取值：1, 2, ..., 100
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // 反过来
    for _i in (1..10).rev(){
        println!("fizzbuzz");
    }

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("我们中间有一个 Rustacean！"),
            // TODO ^ 尝试删除 & 并只匹配 "Ferris"
            _ => println!("你好 {}", name),
        }
    }

    println!("names: {:?}", names);

    // 上面的时借用，这个时直接让所有权消失，但是数据本身没有破坏，只是names失去了对元素的所有权
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("我们中间有一个 Rustacean！"),
            _ => println!("你好 {}", name),
        }
    }

    // `names` has been 'moved' and can no longer be used.
    // Try uncommenting the line below to see the compiler error:
    // println!("names: {:?}", names);


}