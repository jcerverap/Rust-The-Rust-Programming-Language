pub fn do_strings() {

    println!("--- Strings Exercise ---");

    let mut s = String::from("Hello");

    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is moved here and can no longer be used
    println!("{}", s3);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s123 = format!("{s1}-{s2}-{s3}");
    println!("s123 is {s123}");

    for c in "Здравствуйте".chars() {

        println!("{c}");
    }

    for b in "Здравствуйте".bytes() {

        println!("{b}");
    }

    for b in "Здравствуйте".bytes() {
        println!("{:02x}", b);
    }
}