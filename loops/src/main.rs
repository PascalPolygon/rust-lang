fn main() {
    // loop {
    //     println!("again!");
    // }


    /////////////////////////Returning values from loops
    let mut counter = 0;

   let result = loop {
       counter += 1;

       if counter == 10 {
           break counter * 2; //will return counter times 2 after breaking out of loop
           //we use a semicolon to end the statement that assigns the value to result.
       }
   };

   /////////////////Conditional loops with while
   let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

   println!("The result is {}", result);

   //////////////////For loop
   let a = [10, 20, 30, 40, 50];

   for element in a.iter() {
       println!("the value is: {}", element);
   }

   for number in (1..4).rev() { //loops from 3 to 1 (1--4) is notation enabled by range type from stdlib
                                //rev() reverses the range
       println!("{}!", number);
   }
   println!("LIFTOFF!!!");
}
