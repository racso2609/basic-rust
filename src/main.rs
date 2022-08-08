const MAXIMUM_NUMBER: u8 = 5;

enum Direction {
    // Up,
    Down,
    // Left,
    // Right,
}

fn main() {
    // comments
    // i32
    // i64
    // f32
    // f64
    // bool
    //
    let mut x: i32 = 30;

    loop {
        println!("hello {}", x);

        if x < 40 {
            x += 1;
        } else {
            break;
        }
    }

    println!("while");
    while x < 50 {
        x += 1;
        println!("hello {}", x);
    }

    println!("for");
    for y in 1..11 {
        println!("hello {}", x + y);
    }

    println!("interact with arrays");
    let animals: Vec<&str> = vec!["chicken", "Rabbit"];
    for (index, animal) in animals.iter().enumerate() {
        println!("{} {}", animal, index);
    }

    println!("enum");
    let player_direction: Direction = Direction::Down;
    match player_direction {
        // Direction::Up => println!("zucaritas Up"),
        Direction::Down => println!("zucaritas Down"),
        // Direction::Right => println!("zucaritas Right"),
        // Direction::Left => println!("zucaritas Left"),
    }
    println!("constant");
    for x in 1..MAXIMUM_NUMBER {
        println!("{}", x);
    }

    println!("tuple");

    // can storage different value types
    let tup1 = (20, 25, 30);
    let (a, b, _c) = tup1;

    println!("{} {} {}", a, b, tup1.2);

    // functions
    print_hello(2);

    // references
    // let object = 10;
    // let reference = &x;
}

fn print_hello(mut iteration_number: i32) {
    iteration_number += 1;
    for _x in 1..iteration_number {
        println!("Hello");
    }
}
