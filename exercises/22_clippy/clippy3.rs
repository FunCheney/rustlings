// Here are some more easy Clippy fixes so you can see its utility 📎
// TODO: Fix all the Clippy lints.

#[rustfmt::skip]
#[allow(unused_variables)]
fn main() {
    // 原来用 is_none + unwrap 是反了，应该用 if let 来安全处理 Option
    let my_option: Option<&str> = None;
    if let Some(value) = my_option {
        println!("{value}");
    } else {
        println!("my_option 是 None"); // 明确处理 None 的情况
    }

    // 原来的数组缺少逗号导致语法怪异，修复为合法切片
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {my_arr:?}");

    // resize 返回 ()，原来赋值写法是错误的。想要清空，用 clear() 或直接构造空 vec
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {my_empty_vec:?}");

    // 交换两个值用 std::mem::swap 更清晰，避免先后覆盖导致逻辑错乱
    let mut value_a = 45;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");
}