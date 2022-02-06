use std::mem::drop;

fn main() {
    let a: i32 = 40;
    let b: Box<i32> = Box::new(60);
    let c = a + *b;
    // let d = a + b; // error
    println!("a = {:?}, b = {:?}", a, &b); // print is ok, but implement operator have to using pointer (dereferece)
    println!("total: {:?}", c);

    println!("============================");
    let d = Box::new(1); // allocate value on the heap
    let e = Box::new(2);
    let f = Box::new(3);
    let result_1 = *d + *e + *f;

    drop(d); // The memory holding `d` is now available

    let d = Box::new(4);
    let result_2= *d + *e + *f;
    println!("{}, {}", result_1, result_2);
}
