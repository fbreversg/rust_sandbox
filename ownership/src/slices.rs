fn slices(){
    // String slices
    let s = String::from("Hello world");
    let hello = &s[0..5];  // can be rewritten as [..5]
    let world = &s[6..11]; // can be rewritten as [6..]

    let len = s.len();
    let slice =  &s[0..len];
    let slice = &s[..];

    let word = first_word(&s);

    s.clear();  // This throws a compilation error that avoids a runtime error with following print
                // If there's a inmutable reference, can't also take a mutable one   

    println!("the first word in {} is {}", s, word);
}

fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }
    &s[..]
}

fn first_word_str_slice(s: &str) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }
    &s[..]
}