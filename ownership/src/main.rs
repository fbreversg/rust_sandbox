fn main() {
    string_heap_stack();
    copy_reference();

    let s = String::from("hello"); 
    takes_ownership(s);  // s's value moves into function
                                    // ... no valid longer here

    let x = 5;
    makes_copy(x);  // x moves into function but is copied
                                // still valid here
    
    let s1 = gives_ownership(); // function gives moves its return and gives ownership to s1

    let s2 = String::from("hello"); // s2 comes to scope
    let s3 = takes_and_gives_ownership(s2); // s2 is moved to function and return is moved to s3
}   // s was moved nothing happens
    // x goes out of scope
    // s1 is dropped
    // s2 was moved nothing happens
    // s3 is dropped


fn string_heap_stack() {
    let s_literal = "hello"; // Goes to stack
    
    let mut s = String::from("hello"); // Goes to heap
    s.push_str(", world!");
    println!("{}",s);
}

fn copy_reference() {
    let x = 5;
    let y = x; // Copy

    let s1 = String::from("hello");
    let s2 = s1; // Reference (invalidates s1)

    //println!("{}, world!", s1); // value borrowed error
    // Rust will never automatically create “deep” copies of your data

    let s3 = s2.clone();
    println!("{}, {}", s2, s3);
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);        
} // some_string is dropped

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope, nothing happens

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // returned and moved to calling function
}

fn takes_and_gives_ownership(a_string: String) -> String {
    a_string    // returned and moved to calling function
}