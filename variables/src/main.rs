fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x if: {}", x);
    shadowing();
    shadowing_vs_let();
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x is: {}", x);
    }

    println!("The value of x is: {}", x);
}

fn shadowing_vs_let() {
    let spaces = "    ";
    let spaces = spaces.len();

    // But not
    // let mut spaces = "     ";
    // let spaces = spaces.len();
}
