fn main() {
    let x = 5;
    let y = Box::new(x);
    let z = vec!["Hi there"];
    let zz = Box::new(z);

    println!("{:?}", zz);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
