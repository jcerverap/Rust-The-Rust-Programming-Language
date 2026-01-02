pub fn do_ownership() {
    println!("Ownership Exercise:");

    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2

    // println!("{}", s1); // This would cause a compile-time error

    let s3 = s2.clone(); // s2 is cloned to s3
    println!("s2 = {}, s3 = {}", s2, s3);

    let x = 5;
    let y = x; // Copy trait, x is still valid
    println!("x = {}, y = {}", x, y);

    let s4 = String::from("ownership");
    takes_ownership(s4); // s4 is moved into the function

    let x1 = 10;
    makes_copy(x1); // x1 is copied into the function
    println!("x1 after function call: {}", x1);

    let s4 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let s5 = String::from("hello");    // s5 comes into scope

    let s6 = takes_and_gives_back(s5); // s5 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s6
    println!("s6 = {}", s6);

    let (mut s7, len) = calculate_length(s6);
    println!("The length of '{}' is {}.", s7, len);

    // References and Borrowing
    //-------------------------

    let len2 = calculate_length2(&s7);
    println!("The length of '{s7}' is {len2}.");

    // Mutable References
    //-------------------
    let mut s8 = String::from("hello");
    change(&mut s8);
    println!("s8 after change: {}", s8);


    let r1 = &s8; // no problem
    let r2 = &s8; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s8; // no problem
    println!("{r3}");

    //Dangling References
    //-------------------

    let new_to_string = no_dangle();
    println!("new_to_string: {}", new_to_string);

    let my_string = String::from("hello world");
    let word = first_word(&my_string);
    let second_word = &my_string[word.len()..];
    
    println!("The first word is: {}", word);
    println!("The second word is: {}", second_word);

}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn no_dangle() -> String {

    let s = String::from("hello");
    s
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}