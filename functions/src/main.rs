fn main() {
    another_function(5);
    yet_another_function(5, 'h');
    nested_expression();
    println!("Value of five is {}", five());
    let x = plus_one(5);
    println!("Value of 5 plus one is: {}", x);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn yet_another_function(x: i32, label: char) {
    println!("{}, {}", x, label);
}

fn nested_expression() {
    let y = {
        let x = 3;
        x + 1 // not ; (if ; is added it's an statement, not an expression)
    };

    println!("The value of y is: {}", y);
}

fn five() -> u32 {
    // Blowing my mind xD
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
