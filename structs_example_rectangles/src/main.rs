#[derive(Debug)] //In order to be able to use :? specifier for debug struct formatting

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    // println!("rect1 is {}", rect1); //THis would not work because Rust would not know how to
                                    //format the struct before displaying it
    //We need to specify which display format, we will use the debug format
    println!("rect1 is {:?}", rect1); //the :? sepcifier for debug formatting is enabled by
                                     // the first line of code above.

    println!("rect1 is {:#?}", rect1); //:#? is the pretty-print specifer, makes it neater
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1) //passing a reference to to rect1 to area function
    );
}

//we want to borrow the struct rather than take ownership of it.
//This way, main retains its ownership and can continue using rect1
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
