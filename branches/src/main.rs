fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //Because if is an expression, we can use it on the right side of a let statement, as in Listing 3-2.
    let condition = true;
    let number = if condition { // if is an expression in Rust not a statement
        5
    } else {
        6 // values in each branch of an if expression must be of the same data types
        "six" //This would not compile
    };

    println!("The value of number is: {}", number);
}
