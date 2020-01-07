fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    //Statements are instructions that perform some action and do not return a value.
    //Expressions evaluate to a resulting value. Let’s look at some examples.

    //////////EXPRESSION example
    let x = 5;

    let y = {
        let x = 3;
        x + 1 //expressions do not include ending semi colons. Adding a semi colon will turn it
             // into a statement, and you'll get an error
    };

    println!("The value of y is: {}", y); //y will evaluate to 4

    ///////////FUNCTIONS WITH RETURN value
    let x = five();

    println!("The value of x is: {}", x);

    let x = plus_one(5);

   println!("The value of x is: {}", x);
}

// fn another_function() {
//     println!("Another function.");
// }

fn another_function(x: i32, y: i32) {
    print!("The value of x is: {}\n", x);
    println!("The value of y is: {}", y);
}

//example of function with return value. Implicitly returns last expression
//no return keyword required.
fn five() -> i32 {
    5 //this is valid in Rust (no semicolon again because we want to return this expressio)
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
