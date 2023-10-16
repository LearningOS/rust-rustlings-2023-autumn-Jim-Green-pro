// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



#[allow(unused_variables, unused_assignments)]


fn main() {
    // 1. Fix the missing comma in the array definition
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 2. Remove the unnecessary unwrap
    // Since we know my_option is None, there's no need to unwrap it
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // Do something else, but don't unwrap
    }

    // 3. Use std::mem::swap to swap the values of value_a and value_b
    let mut value_a = 45;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);

    // 4. Clear the vector instead of resizing it to zero
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);
}