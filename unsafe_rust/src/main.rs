use std::slice;

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

extern "C" {
    fn abs(input: i32) -> i32;
}
unsafe fn dangerous() {}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("called a rust function from C!");
}

unsafe trait Foo {
    // unsafe methods
}

unsafe impl Foo for i32 {
    // method implementations
}

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("name: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        dangerous();

        println!("|-3| = {}", abs(-3));
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
