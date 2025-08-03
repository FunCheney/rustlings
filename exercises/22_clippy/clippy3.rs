// Here are some more easy Clippy fixes so you can see its utility 📎
// TODO: Fix all the Clippy lints.

#[rustfmt::skip]
#[allow(unused_variables)]
fn main() {
    let my_option: Option<&str> = None;
    // Safely handle Option using pattern matching
    if let Some(val) = my_option {
        println!("{}", val);
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    // Create mutable vector and resize it
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.resize(0, 5);
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Properly swap values using tuple assignment
    (value_a, value_b) = (value_b, value_a);
    println!("value a: {value_a}; value b: {value_b}");
}