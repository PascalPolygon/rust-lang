//Using match control flow to get cent value of a coin
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

//with if, the expression needs to return a Boolean value, but here, it can be any type. The type
//of coin in this example is the Coin enum that we defined on line 1.
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

//the following code would print “Lucky penny!” every time the method was called with a Coin::Penny
//but would still return the last value of the block, 1
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

//In the match expression for this code, we add a variable called state to the pattern that matches
//values of the variant Coin::Quarter. When a Coin::Quarter matches, the state variable will bind to
//the value of that quarter’s state. Then we can use state in the code for that arm, like so:
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
//If we were to call value_in_cents(Coin::Quarter(UsState::Alaska)), coin would be
//Coin::Quarter(UsState::Alaska). When we compare that value with each of the match arms, none of
//them match until we reach Coin::Quarter(state). At that point, the binding for state will be the
//value UsState::Alaska.

//Matching with Option<T>

//Let’s say we want to write a function that takes an Option<i32> and, if there’s a value inside, adds
//1 to that value. If there isn’t a value inside, the function should return the None value and not attempt
//to perform any operations.

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

//First call of plus_one
//Does Some(5) match Some(i)? Why yes it does! We have the same variant. The i binds to the value
//contained in Some, so i takes the value 5. The code in the match arm is then executed, so we add 1
//to the value of i and create a new Some value with our total 6 inside.

//Second call of plus_one
//None => None matches!

//Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid
