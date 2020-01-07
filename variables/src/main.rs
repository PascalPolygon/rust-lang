fn main() {

    ///////////////////////////Mutability
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    //////////////////////Shadowing
    // let x = 5;
    // let x = x + 1; //shadows by redeclaring variable of the same name
    //Showding lets you do transformations on a variable while the variable remains immutable
    // let x = x * 2;
    //Shadowing also lets you change datatypes without having to rename variables
    //Impossible to mutate a variable's type (couldn't do this if spaces was mutable)
    // let spaces = "   ";
    // let spaces = spaces.len();
    //
    // println!("The value of x is: {}", x);

    /////////////////////Data types
    //Rust is a statically typed language,
    //which means that it must know the types of all variables at compile time.
    // Rust’s defaults are generally good choices, and integer types default to i32:
    //this type is generally the fastest, even on 64-bit systems.

    ////////////////////Tuples
     let tup: (i32, f64, u8) = (500, 6.4, 1); //compound datatype to store many different types in concept
     // let tup = (500, 6.4, 1); //Can also be written like this without explicit type annotations
     //To destructure a tuple and get values out using pattern matching
     let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    //Tuple elements can also be accessed using the . operator
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    ////////////////////Arrays
    //Arrays in Rust are different from arrays in some other languages
    //because arrays in Rust have a fixed length, like tuples.
    //Use vectors if you want your data to grow and shrink
    //if you want to create an array that contains the same value for each element,
    //you can specify the initial value, followed by a semicolon,
    //and then the length of the array in square brackets, as shown here:
    let a = [3; 5];


}
