
#![allow(unused_variables)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
};    //Sample struct definiton

fn main() {

let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
}; //creating an instance of a struct.
// Note that the fields don't have to be in exact same order

let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
//Changing value of a field in mutable struct instance

let mut user2 = build_user("new@rust.com", "true_born_code_writer");

//Struct update syntax
let user3 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active, //setting field value from a different instance of User
    sign_in_count: user1.sign_in_count,
};
//MORE UPDATE SYNTAX
let user4 = User {
    email: String::from("another@example.com"), //email and username have different values
    username: String::from("anotherusername567"), //from user1
    ..user1 //Specifies the remaining values i.e: active and sign_in_count
            //should have the same value as user1
};

//TUPLE STRUCTS TO CREATE DIFFERENT TYPES

//Tuple structs have the added meaning the struct name provides but don’t have names associated with
//their fields; rather, they just have the types of the fields. Tuple structs are useful when you want to give
// the whole tuple a name and make the tuple be a
//different type from other tuples, and naming each field as in a regular struct would be verbose or redundant.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

//Note that the black and origin values are different types,
//because they’re instances of different tuple structs.
//For example:  a function that takes a parameter of type Color cannot take a Point as an argument,
// even though both types are made up of three i32 values.

//Unit-like Structs Without Any fields
//You can also define structs that don’t have any fields! These are called unit-like structs because t
//hey behave similarly to (), the unit type. Unit-like structs can be useful in situations in which you need
//to implement a trait on some
//type but don’t have any data that you want to store in the type itself. We’ll discuss traits in Chapter 10.

}


fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
} //function to create instance of user

//FIELD INIT SHORTHAND
fn build_user(email: String, username: String) -> User {
    User {
        email, //same as above, but with field init shorthand (shorter than email:email)
        username, //fn parameters and struct fields must have the same name
        active: true,
        sign_in_count: 1,
    }
}

//Side Note
//Trying to define a struct like this:
struct User {
    username: &str, //with &str data type as opposed to owned String data type
    email: &str,
    sign_in_count: u64,
    active: bool,
}
//...Will not work when you try to assign values to to the field like:
fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
//This will not work because you are trying to store references to data owned by something else
//Remember that &str is a reference type. This is possible but you need to use lifetimes which
//Which is discussed in chapter 10
