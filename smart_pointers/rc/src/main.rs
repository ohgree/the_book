use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Ref count: {}", Rc::strong_count(&a));
    let b = Rc::new(Cons(3, Rc::clone(&a)));
    println!("Ref count: {}", Rc::strong_count(&a));
    {
        let c = Rc::new(Cons(4, Rc::clone(&a)));
        println!("Ref count: {}", Rc::strong_count(&a));
    }
    println!("Ref count: {}", Rc::strong_count(&a));
}
