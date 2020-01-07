
#![allow(unused_variables)]
fn main()
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s

    let s = String::from("hello"); //This is how to create a string from a string literal
    //The default string litteral is immutable, and its size can't be changed
    //the String type allows us to allocate memory on the heap so that you can store
    // an ammount of text that isn't known at compile time (dynamic mem alloc like malloc)
    // we create a string from a literal using from

    //now s can be mutated
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    ////////WAYS VARIABLES AND DATA INTERACT: MOVE
    //moving s1 to s2
    let s1 = String::from("hello");
    let s2 = s1; //Deos not create a copy of s1 at a different spot on heap instead only copies stack DATA
                //That is meta data (pointer to heap location, length, and size). Then because you can't have
                //2 owners for the same memory to avoid double frees
                //Rust invalidates s1 as owner of that memory location.
                //In rust this is not called a shallow copy but a move.
                //A consequence of this is performance gain as you're not copying potentially large data over and over

    println!("{}, world!", s1); //This would create an error at compile time

    //////////WAYS VARIABLES AND DATA INTERACT: CLONE
    //If you in fact wanted to copy all heap data and do a "deep copy"
    // use the clone method like so:
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    ////////////STACK ONLY DATA: COPY (int, float, char)
    let x = 5; //5 Is an integer type(stored on the stack) which can have a copy trait
    let y = x; //meaning that this operation actually creates a "deep copy" of x on the stack

    println!("x = {}, y = {}", x, y); //Therefore this line valid for types that live on the stack
                                      //tuples comprised uniquely of stack data also has the copy trait
}                      // this scope is now over, and s is no longer valid

/////////////OWNERSHIP AND FUNCTIONS
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

////////////////RETURN VALUES AND OWNERSHIP
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

////////////////IT'S POSSIBLE TO RETURN MULTIPLE VALUES USING A TUPLE
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

//////////////REFERENCES AND BORROWING
//The issue with the tuple code is that s1 is unavailable in main when you give ownership
// to calculate_length function by passing it as an argument. for s1 to rebecome available in must be
//'given' back to main from the function and stored in the tuple as s2. In contrast consider the following code.
//We call having references as function parameters borrowing.
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); //using the & operator we pass a reference to s1 as oposed to moving s1
                                    // to calculate_length. Now s1 is valid in main but we can still use its DATA
                                    // in the function because it has a reference to it. No need to return a tuple
                                    // to store the string as s2 to rescope within the main

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { //These ampersands are references, and they allow you to refer to
                                         //some value without taking ownership of it. Figure 4-5 shows a diagram.
    s.len()
    //notice that we don't need to return s here because this function never had ownsership of it
    //just a reference. Can't return something you never owned
    //Note that we could not modity s in this function if we wanted to because by default REFERENCES
    //like fucntions are immutable. You would need to pass (s: &mut String) to make it mutable reference
}// Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.(Nothing gets dropped)

//////////////MUTABLE REFERENCES
fn main() {
    let mut s = String::from("hello");

    change(&mut s); //now using &mut s

    //Only one mutable reference to a piece of a data in a given scope allowed
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; //This is not allowed. compile time error because it would create data race condition

    println!("{}, {}", r1, r2);

    // As a work-around we can create a different scope for mutliple mutable REFERENCES
    let mut s = String::from("hello");

    {
    let r1 = &mut s;
    }    // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    //////Can't mix mutable and immutable references
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    //cannot borrow `s` as mutable because it is also borrowed as immutable
    // Whew! We also cannot have a mutable reference while we have an immutable one.
    // Users of an immutable reference don’t expect the values to suddenly change out from under them!
    // However, multiple immutable references are okay
    // because no one who is just reading the data has the ability to affect anyone else’s reading of the data.
    println!("{}, {}, and {}", r1, r2, r3);

    // Note that a reference’s scope starts from
    // where it is introduced and continues through the last time that reference is used.
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
    //The scopes of the immutable references r1 and r2 end after
    //the println! where they are last used, which is before the mutable reference r3 is created.
    //So this code is allowed


}

////////////////////DANGLING REFERENCES
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
//Because s is created inside dangle, when the code of dangle is finished, s will be deallocated. But we tried to return a reference to it.
//That means this reference would be pointing to an invalid String. That’s no good! Rust won’t let us do this.


//THe solution is to return the string directly
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}// This will work fine

fn change(some_string: &mut String) { // same here
    some_string.push_str(", world");
}
