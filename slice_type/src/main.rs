// Here’s a small programming problem: write a function that takes a string and returns the first word it finds in
// that string. If the function doesn’t find a space in the string,
// the whole string must be one word, so the entire string should be returned.

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    println!("Value of word: {}", word);

    s.clear(); // this empties the String, making it equal to ""
    println!("the first word is: {}", word);
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    //The code above does not run when first_word method returns a slice


    //As a solution Rust has string slices

    let s = String::from("hello world");

    let hello = &s[0..5];
    let hello = &s[..5]; //equal to line above
    //it's like a reference but only on a portion of the string
    //starting_index is the first position in
    //the slice and ending_index is one more than the last position in the slice.

    let world = &s[6..11]; //So in the case of let world = &s[6..11];, world would be a slice that contains
                           //a pointer to the 7th byte of s with a length value of 5.

    //droping last byte of string if range includes it
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..]; //if range  includes last byte of the string you can drop it too

    //Drop both start and end to take slice of the entire String
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];

    //&String type Vs &str
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    // These will only work with the second definition of first_word below
    let word = first_word(&my_string[..]); //String of type &String

    let my_string_literal = "hello world"; // string literal of type &str

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    //Slices work on more than just strings
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3]; //THis will return a slice of type &[i32]
}

//The first_word function that returns a byte index value into the String parameter
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); //converts string to an array of bytes that can be looped through

    for (i, &item) in bytes.iter().enumerate() {
        // We’ll discuss iterators in more detail in Chapter 13. For now, know that iter is a method that returns
        // each element in a collection and that enumerate wraps the result of iter and returns each element as
        // part of a tuple instead. The first element of the tuple returned from enumerate is the index, and the
        // second element is a reference to the element. This is a bit more convenient than calculating the
        // index ourselves. then (i, &item) deconstructs the tuple
        if item == b' ' { // Finds byte in byte array that represents empty space.
            println!("i: {}", i);              // Using byte literal syntax b'...'
            return i;     //returns index of empty space in byte array

        }
    }

    s.len() // not sure why this returns i
}

//REwrite first_word to return a slice
//The type that signifies “string slice” is written as &str
fn first_word(s: &str) -> &str { // Note that function parameter here is of type &str and not &String.
                                // THis allows us to take both slices and string literals as parameters to this func
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
