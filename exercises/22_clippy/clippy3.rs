// Here are some more easy Clippy fixes so you can see its utility 📎
// TODO: Fix all the Clippy lints.

#[allow(unused_variables, unused_assignments)]
#[allow(clippy::manual_swap)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_some() {
        println!("my_option has a value!");
    } else {
        println!("my_option is None, no value to unwrap.");
    }

    let my_arr = &[-1, -2, -3 - 4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec: Vec<i32> = vec![];

    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    let temp = value_a;
    value_a = value_b;
    value_b = temp;
    println!("value a: {}; value b: {}", value_a, value_b);
}
