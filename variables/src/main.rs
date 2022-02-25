use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x if: {}", x);
    shadowing();
    shadowing_vs_let();
    some_types();
    array_out_bond_example();
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

fn some_types() {
    let x: u32 = 8;
    let j: i32 = -8;
    let y: bool = true;
    let z: f32 = -3.0;
    let z: f64 = -4.0;

    //compound
    // tuples: multiple types, fixed length
    let tup: (i64, bool, u32) = (j.into(), y, x); //casting with into()

    let (a, b, c) = tup;

    println!("The value of b is: {}", b);
    println!("Accesing with .: {}", tup.1);

    //arrays: single type, fixed lenght
    let arr = [1, 2, 3, 4, 5];
    let arr_b: [i32; 5] = [1, 2, 3, 4, 5]; // explicit typing
    let arr_c = [3; 5]; // value; repeat n times = [3, 3, 3, 3, 3]

    let first = arr[0];
    let second = arr[1];
}

fn array_out_bond_example() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    // Invalid index will throw a panic error on runtime
    println!("The value of the element in index {} is {}", index, element);
}
