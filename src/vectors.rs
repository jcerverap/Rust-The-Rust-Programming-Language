pub struct Number {
    value: i32,
}

impl Number {
    pub fn new(value: i32) -> Self {
        Number { value }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

pub fn do_vectors() {

    println!("--- Vectors Exercise ---");

    let mut numbers: Vec<Number> = Vec::new();

    numbers.push(Number::new(10));
    numbers.push(Number::new(20));
    numbers.push(Number::new(30));

    for number in &numbers {
        println!("Number value: {}", number.get_value());
    }

   let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 1;
        println!("{i}");
    }    
}   