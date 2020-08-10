fn main() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("Third is: {}", third);

    match v.get(2) {
        Some(third) => println!("Third is: {}", third),
        None => println!("no third element"),
    };

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 50;
    }
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Text(String::from("asd")),
        SpreadsheetCell::Float(10.12),
    ];
}
