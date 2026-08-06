pub trait Iterator {
    // 被迭代的类型
    type Item;

    // `find` 接受 `&mut self`，这意味着调用者可能被借用
    // 和修改，但不会被消耗。
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
        // `FnMut` 表示任何捕获的变量最多只能被修改，不能被消耗。
        // `&Self::Item` 表示它通过引用将参数传递给闭包。
        P: FnMut(&Self::Item) -> bool;
}

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `vec1.iter()` yields `&i32`.
    let mut iter = vec1.iter();
    // `vec2.into_iter()` yields `i32`.
    let mut into_iter = vec2.into_iter();


    //   第一层：iter() 产生 &i32
    //   vec1.iter()   // 迭代器产生 &i32（每个元素是引用）
    //   所以遍历时，每个元素是 &i32。
    //   第二层：find 的闭包参数是 &Item,可以看find的定义，使用的是&item
    // `iter()` yields `&i32`, and `find` passes `&Item` to the predicate.
    // Since `Item = &i32`, the closure argument has type `&&i32`,
    // which we pattern-match to dereference down to `i32`.
    println!("在 vec1 中查找 2：{:?}", iter.find(|&&x| x == 2));
    
    // `into_iter()` yields `i32`, and `find` passes `&Item` to the predicate.
    // Since `Item = i32`, the closure argument has type `&i32`,
    // which we pattern-match to dereference down to `i32`.
    // into就没有那么多事了，引用后就是i32，它返回值本身，不是引用
    println!("在 vec2 中查找 2：{:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `array1.iter()` yields `&i32`, and `find` passes `&Item` to the
    // predicate. Since `Item = &i32`, the closure argument has type `&&i32`.
    println!("在 array1 中查找 2：{:?}", array1.iter().find(|&&x| x == 2));
    // `array2.into_iter()` yields `i32` (since Rust 2021 edition), and
    // `find` passes `&Item` to the predicate. Since `Item = i32`, the
    // closure argument has type `&i32`.
    println!("在 array2 中查找 2：{:?}", array2.into_iter().find(|&x| x == 2));
}