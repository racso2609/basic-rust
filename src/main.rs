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
}
