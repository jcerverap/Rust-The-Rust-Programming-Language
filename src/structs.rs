//User-defined data types: Structs
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//Unit-like struct
struct AlwaysEqual;

//Regular struct with Debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
} 

//Methods for Rectangle struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

pub fn do_structs() {

    println!("Structs exercise selected.");
 
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        sign_in_count: 0,
        active: true,
    };
    user1.sign_in_count = 1;
    println!("Username: {}, Email: {}", user1.username, user1.email);

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("Username: {}, Email: {}", user2.username, user2.email);

    // Creating from Tuple Structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Color - R: {}, G: {}, B: {}", black.0, black.1, black.2);
    println!("Point - X: {}, Y: {}, Z: {}", origin.0, origin.1, origin.2);

    // Using Unit-like Structs
    let subject = AlwaysEqual;
    println!("Created an instance of a unit-like struct: {:?}", std::any::type_name::<AlwaysEqual>());

    do_example_structs();
    do_methods();
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn do_example_structs() {

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("rect1 is {rect1:?}");


    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);    
}


fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn do_methods() {

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(3);
    println!("Square: {:#?}", square);

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }


}   