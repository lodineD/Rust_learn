

// 这个例子不是"重新实现" any，而是展示 Iterator 里内置的 any 方法怎么用——以及 iter() 和 into_iter()的区别。
pub trait Iterator {
    // 被迭代的类型
    type Item;

    // `any` 接受 `&mut self`，意味着调用者可能被借用
    // 和修改，但不会被消耗
    fn any<F>(&mut self, f: F) -> bool where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `Self::Item` is the closure parameter type,
        // which is determined by the iterator (e.g., `&T` for `.iter()`,
        // `T` for `.into_iter()`).
        F: FnMut(Self::Item) -> bool;
}

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec 使用 `iter()` 产生 `&i32`，解构为 `i32`
    println!("2 在 vec1 中：{}", vec1.iter()     .any(|&x| x == 2));
    // 对 vec 使用 `into_iter()` 产生 `i32`，无需解构
    println!("2 在 vec2 中：{}", vec2.into_iter().any(|x| x == 2));

    // `iter()` 只借用 `vec1` 及其元素，所以它们可以再次使用
    println!("vec1 长度：{}", vec1.len());
    println!("vec1 的第一个元素是：{}", vec1[0]);
    // `into_iter()` 会移动 `vec2` 及其元素，所以它们不能再次使用
    // println!("vec2 的第一个元素是：{}", vec2[0]);
    // println!("vec2 长度：{}", vec2.len());
    // TODO：取消上面两行的注释，观察编译器错误

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组使用 `iter()` 产生 `&i32`，使用借用
    println!("2 在 array1 中：{}", array1.iter()     .any(|&x| x == 2));
    // 对数组使用 `into_iter()` 产生 `i32`
    println!("2 在 array2 中：{}", array2.into_iter().any(|x| x == 2));
    //  ┌──────────────────┬────────────────────────┬──────────────┐
  //    │       类型        │    into_iter() 行为    │ 之后还能用吗 │
  //    ├──────────────────┼────────────────────────┼──────────────┤
  //    │ Vec<i32>         │ 拿走所有权             │      ❌      │
  //    ├──────────────────┼────────────────────────┼──────────────┤
  //    │ [i32; 3]（数组）  │ 不拿所有权，相当于拷贝 │      ✅      │
  //    └──────────────────┴────────────────────────┴──────────────┘

  // 原因：数组是 Copy 类型的（固定的、栈上的），into_iter() 对数组来说不会移动所有权，而是产生一个"值迭代器"。Vec 不是
  // Copy，into_iter() 会拿走所有权。
  // Rust 的默认行为：对于 Copy 类型（i32、bool、f64、数组/元组只含 Copy
  // 元素），默认是按位拷贝（可以理解为浅拷贝，但没有指针问题）；对于非 Copy 类型（String、Vec），默认是移动所有权，不拷贝。
}