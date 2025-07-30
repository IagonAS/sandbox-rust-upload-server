use std::fs;

#[derive(Clone)]
struct User {
    age: u8,
    active: bool,
    username: String
}

struct Rect {
    width: u8,
    height: u8,
}

impl Rect {
    fn area(&self) -> u8 {
        self.width * self.height
    }
}

enum Direction {
    Left,
    Right
}

enum Shape {
    Circle(f32),
    Square(f32),
    Rectangle(f32, f32),
}

pub fn run_all_examples() {

    // Primitives
    let x: i8 = -5;
    let y: u8 = 100;
    let z: f32 = 1000.001;

    let greeting = String::from("Hello, world!");
    println!("x:{x}, y:{y}, z:{z}, {greeting}");

    // Mutable variable
    let mut is_above_18: bool = true;

    is_above_18 = false;

    // Conditions
    if is_above_18{
    println!("Above 18");
    } else {
    println!("Below 18")
    }

    // Iterations
    for i in 0..3 {
    println!("{i}");
    }

    let first_world = get_first_word(String::from("This is test"));

    println!("first world: {first_world}" );
    println!("sum: {}", do_sum(5, 6));

    // Borrowing & references
    let str_to_borrow = String::from("This str will be borrowed");
    let reference = &str_to_borrow; // borrowing, not passing
    borrow_data(&str_to_borrow);
    println!("{str_to_borrow}");

    // Mutable reference
    let mut str_to_update = String::from("Hello");
    update_str(&mut str_to_update);
    println!("str_to_update: {str_to_update}");

    // Structs & traits
    let user = User{
    age: 26,
    active: true,
    username: String::from("Test user")
    };

    print_name(user);

    // Enums
    let rect = Rect{width: 10, height: 3};
    println!("react area: {}", rect.area());

    take_direction(Direction::Left);

    let circle = Shape::Circle(1.0);
    let circle_area = calculate_area(circle);
    println!("circle area: {}", circle_area);

    // Error catching
    let res = fs::read_to_string("hello.txt");

    match res{
    Ok(file) => println!("file: {}", file),
    Err(error) => println!("error: {}", error)
    }

    // Option
    let first_a_res = find_first_a(String::from("hallo"));

    match first_a_res {
    Some(idx) => println!("'a' has {idx} index"),
    None => println!("No 'a' found")
    }
}

fn find_first_a(str: String) -> Option<usize> {
    for (idx, char) in str.chars().enumerate() {
        if char == 'a'{
            return Some(idx);
        }
    }
    None
}

fn calculate_area(shape: Shape) -> f32 {
    match shape {
        Shape::Circle(radius) => 3.14 * radius,
        Shape::Square(side) => side * side,
        Shape::Rectangle(width,height) => width * height,
    }
}

fn take_direction(direction: Direction) {}

fn print_name(user: User){
    println!("{}", user.username);
}

fn borrow_data(str: &String){
    println!("Borrowed {str}");
}

fn get_first_word(sentence: String)-> String{
    let mut ans = String::from("");

    for char in sentence.chars(){
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    ans
}

fn do_sum(a: i8, b: i8) -> i8 {
    a + b
}

fn update_str(str: &mut String){
    str.push_str(" world")
}