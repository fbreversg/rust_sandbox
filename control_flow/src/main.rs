fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // No implicit casting: if number {} will raise an error
    // ^^^^^^ expected `bool`, found integer

    if number % 5 == 0 {
        println!("bla");
    } else if number % 4 == 0 {
        println!("ble");
    } else if number % 3 == 0 {
        println!("bli");
    } else {
        println!("blo");
    }

    // Conditional assigment
    let condition = true;
    let a = if condition { 5 } else { 6 };
    println!("{}", a);

    // loops

    loop {
        println!("infinite");
        // or not
        break;
    }

    let mut count = 0;
    //labeled loop o_O
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break; // break inner loop
            }
            if count == 2 {
                break 'counting_up; // break labeled loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // loop that returns a value
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // conditional loops (while)
    let mut number = 3;

    while number != 3 {
        println!("{}", number);
        number -= 1;
    }

    // for loops
    let a = [1, 2, 3, 4, 5];

    for element in a {
        println("{}", element);
    }

    // reverse example
    for number in (1..4).rev() {
        println("{}", number);
    }
}
