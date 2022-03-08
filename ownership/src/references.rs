fn references(){
    let s1 = String::from("hello");

    let len = calculate(&s1); // Using reference doesn't give ownwership, borrowing

    println!("The lenght of {} is {}", s1, len);

    // mutable reference
    let mut s2 = String::from ("world");
    change(&mut s2);

    // only 1 mutable reference at the same time
    let r1 = &mut s2;
    let r2 = &mut s2;

    println!("{},{}", r1, r2);

    // Can be used in scope but not simultaneously
    {
        let r3 = &mut s2;
    }
    let r4 = &mut s2;

    // Not possible to mix mutable and inmutable
    let r5 = &s2;
    let r6 = &s2;
    let r7 = &mut s2; // This is a problem

    println!("{},{},{}",r5, r6, r7);

}

fn calculate(s: &String) -> usize { // parameter as reference
    s.len()
}   // s goes out of scope but since it's a reference nothing happens

fn change(some_string: &mut String) {
    some_string.push_str("!");
}