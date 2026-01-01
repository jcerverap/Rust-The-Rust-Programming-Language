

pub fn do_functions() {

    println!("Functions Exercise:");

    another_function(5);
    print_labeled_measurement(10, 's');
    expression_function();
    return_value_function();

}

fn another_function(x: i32) {

    println!("Another function called with parameter: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expression_function()  {

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn return_value_function() {

    let x = 5;
    let y = plus_one(x);

    println!("The value of y after adding one to x (which is {}) is: {}", x, y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}