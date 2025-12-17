

pub fn do_data_types() {

    println!("Data Types Exercise:");

    let my_tup: (&str, i32, f64, u8) = ("this is my tuple", 500, 6.4, 1);
    println!("My tuple values: {}, {}, {}, {}", my_tup.0, my_tup.1, my_tup.2, my_tup.3);


    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
                "August", "September", "October", "November", "December"];

    println!("Months:");
    for month in months.iter() {
        println!("{} ", month);
        
    }

    let my_array: [u8; 5] = [1, 2, 3, 4, 5];
    println!("My array values: {:?}, length: {}", my_array, my_array.len());

    for i in 0..my_array.len() {
        println!("Element {}: {}", i, my_array[i]);
    }

    do_index();

}

fn do_index() {

    let my_array: [u8; 5] = [1, 2, 3, 4, 5];
    let mut s_index: String = String::new();

    loop {

        println!("Please enter a numeric array index (\'x\' to exit).");

        std::io::stdin()
            .read_line(&mut s_index)
            .expect("Failed to read line");

        s_index = s_index.trim().to_string();

        if s_index == "x" {
            println!("Exiting index exercise.");
            break;
        }

        match s_index.parse::<i32>() {
            Ok(num) => println!("Element at index {}: {}", num, my_array[num as usize]),
            Err(_) => println!("'{}' is not a number", s_index),
        }

        s_index.clear();
    }
}