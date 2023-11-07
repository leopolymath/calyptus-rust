fn add(x: i32, y: i32) -> i32 {

    x + y // No semicolon since it is a return expression
    
}

struct Rectangle {
    width: u32, height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

}

fn main() {
    let a: i32 = 10;
    let b: i32 = 15;
    println!("Hello, world!, {} {}", a, b);

    let number = 3;

    match number { 
        1 => println!("One"), 
        2 => println!("Two"), 
        3 => println!("Three"), 
        4 => println!("Four"), 
        5 => println!("Five"), 
        _ => println!("It's something else"),
    }

    let sum = add(5, 10);
    
    println!("The sum is {}", sum);

    let rect = Rectangle { width: 30, height: 50 };

    println!("The area of the rectangle is {}", rect.area());
}
