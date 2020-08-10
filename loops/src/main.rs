fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result: {}", result);

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("{}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("IterVal: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("Liftoff!");
}
