/*
outer: 和 'inner: 是 loop 标签（loop label）——它们是 Rust 给循环起的一个"名字"，让 break 和 continue
  能精确指定跳出哪个循环。我讲清楚。

  先纠正一个小点：outer 和 inner 不是字段，是"标签"

  outer 和 inner 不是字段、不是变量，是标签（label）——一个给循环起的名字。就像给循环贴个名牌。

  为什么需要标签？

  因为循环可以嵌套（一个循环套另一个）。当有多个循环时，break
  默认只跳出最近的那一层。但有时候你想跳出外层的循环，这时就需要标签来"指名道姓"。
*/


#![allow(unreachable_code, unused_labels)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");

    let mut counter = 0;

    // 如果break成功则会返回一个值，这里就是20
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}