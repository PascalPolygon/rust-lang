//methods are different from functions in that they’re defined within the context of a struct
//(or an enum or a trait object, which we cover in Chapters 6 and 17, respectively), and their
//first parameter is always self, which represents the instance of the struct the method is being called on.

#[derive(Debug)] //In order to be able to use :? specifier for debug struct formatting

struct Rectangle {
    width: u32,
    height: u32,
}

//Example of area method implemented directly on the struct
impl Rectangle {
    fn area(&self) -> u32 { //This method is immutably borrowing self, thus the &.
                            //But we could also take owernship or borrow mutably (&mut self) like with any other param
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool { //will compare dimensions of instance on which it was called
                                                    //(&self) to reference of instance which was passed as a parameter
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
      Rectangle { width: size, height: size }
  } //Example of an associated function (not method) which does not take &self as a parameter.
    //Can be thought of as a contructor. They are often used for constructors. This makes it easy
   //To create a sqaure from a rectangle
}

//Breaking implement block above into 3 seperate implement blocks is valid syntax

// The main benefit of using methods instead of functions, in addition to using method syntax and not having
//to repeat the type of self in every method’s signature, is for organization. We’ve put all the things we can
//do with an instance of a type in one impl block rather than making
//future users of our code search for capabilities of Rectangle in various places in the library we provide.
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("rect1 is {:#?}", rect1); //:#? is the pretty-print specifer, makes it neater

    println!(
        "The area of the rectangle is {} square pixels.",
         rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); //calling can_hold method on rect1 instance
                                                                 // that is &self will be &rect1
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    //Rust doesn't need the -> operator as in C/C++ because it has a feature called
    //automatic referencing and dereferencing. Where writing:
    //p1.distance(&p2); and
    //(&p1).distance(&p2); is the same

    let sq = Rectangle::square(3); //Returns an instance of Rectangle struct that is a square

    // Methods let you specify the behavior that instances of your structs have, and associated
    //functions let you namespace functionality that is particular to your struct without having an
    // instance available.
}

//we want to borrow the struct rather than take ownership of it.
//This way, main retains its ownership and can continue using rect1
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
