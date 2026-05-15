use std::result;

fn divine(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    }
    else {
        Some(a / b)
    }
}

fn main() {

    match divine(10, 0) {
        Some(result) => println!("Result: {}", result),
        None              => println!("Divine by zero!")
    }

    let result = divine(10, 0).unwrap_or(-1);
    println!("Result: {}", result);
}
