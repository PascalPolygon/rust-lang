enum IpAddrKind { //because enum values can only be one of the variants.
    V4, // V4 and V6 are known as the variants of the enum
    V6,
}

//Instead of using structs to store ip data as such:
struct IpAddr { //Struct declaration
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr { //First instance of struct
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr { //Second instance of struct
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};

//We can represent the same concept in a more concise way using just an enum, rather than an enum
//inside a struct, by putting data directly into each enum variant.

enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));

//There’s another advantage to using an enum rather than a struct: each variant can have different
//types and amounts of associated data. Version four type IP
//If we wanted to store V4 addresses as four u8 values but still express V6 addresses as one String
//value, we wouldn’t be able to with a struct. Enums handle this case with ease:

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));

//Let’s look at how the standard library defines IpAddr: it has the exact enum and variants that
//we’ve defined and used, but it embeds the address data inside the variants in the form of two different
//structs, which are defined differently for each variant:
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
//This code illustrates that you can put any kind of data inside an enum variant: strings, numeric
//types, or structs, for example. You can even include another enum!

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
//A Message enum whose variants each store different amounts and types of values

//The following structs could hold the same data that the preceding enum variants hold:

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

//But if we used the different structs, which each have their own type, we couldn’t as easily d
//efine a function to take any of these kinds of messages as we could with the Message enum defined
//in Listing 6-2, which is a single type.

//we’re also able to define methods on enums. Here’s a method named call that we could define on
//our Message enum:

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();

//The problem with null values is that if you try to use a null value as a not-null value, you’ll
//get an error of some kind. Because this null or not-null property is pervasive, it’s extremely easy
//to make this kind of error.
//So rust doesn't have a null type in the Option enum it has Some and None

//Option enum definition by standard library
enum Option<T> {
    Some(T),
    None,
}

//The <T> syntax is a feature of Rust we haven’t talked about yet. It’s a generic type parameter,
//and we’ll cover generics in more detail in Chapter 10. For now, all you need to know is that <T>
//means the Some variant of the Option enum can hold one piece of data of any type. Here are some examples
//of using Option values to hold number types and string types:
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;

//So why is having Option<T> any better than having null?
//In short, because Option<T> and T (where T can be any type) are different types, the compiler won’t
//let us use an Option<T> value as if it were definitely a valid value. For example, this code won’t compile
//because it’s trying to add an i8 to an Option<i8>:

let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y; //This won't work because i8 and Option<i8> are not the same type

//In other words, you have to convert an Option<T> to a T before you can perform T operations with it.
//Generally, this helps catch one of the most common issues with null: assuming that something isn’t null
//when it actually is.


fn main() {
    println!("Hello, world!");
}
