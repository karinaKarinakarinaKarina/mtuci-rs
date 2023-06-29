// fn main() {
//     println!("Hello world!");
// }



// // second chapter, guessing game

// use rand::Rng;
// use std::cmp::Ordering;
use std::io;
// fn main() {
//     println!("Guess the number!");

//     let secret_number = rand::thread_rng().gen_range(1..=100);

//     loop {
//         println!("pleeease... input your guuuess senpaaai...");

//         let mut guess =  String::new();

//         io::stdin().read_line(&mut guess).expect("Failed to read this line bruh");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue
//         };

//         println!("Senpai! You guessed: {guess}");

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("its too small, senpai~"), 
//             Ordering::Greater => println!("senpaai~, it's too big"), 
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }   
// }


// // third chapter -  Common Programming Concepts

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");


//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


//     let y = 5;
//     let y = y + 1;
//     {
//         let y = x * 2;
//         println!("The value of x in the inner scope is: {y}");
//     }
//     println!("The value of x is: {y}");


//         // addition
//     let sum = 5 + 10;
//     // subtraction
//     let difference = 95.5 - 4.3;
//     // multiplication
//     let product = 4 * 30;
//     // division
//     let quotient = 56.7 / 32.2;
//     let truncated = -5 / 3; // Results in -1
//     // remainder
//     let remainder = 43 % 5;
    

//     let tup = (500, 6.4, 1);
//     let (g, j, m) = tup;
//     println!("The value of y is: {m}");

//     let a: [i32; 5] = [1, 2, 3, 4, 5];
//     println!("Please enter an array index.");
//     let mut index = String::new();
//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");
//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");
//     let element = a[index];
//     println!("The value of the element at index {index} is: {element}");


//     let condition = true;
//     let number = if condition { 5 } else { 6 };
//     println!("The value of number is: {number}");


//     let mut counter = 0;
//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//     println!("The result is {result}");


//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;
//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//         count += 1;
//     }
//     println!("End count = {count}");
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }



// // fourth chapter 

// fn main() {
//     let mut s = String::from("hello");
//     s.push_str(", world!"); // push_str() appends a literal to a String
//     println!("{}", s); // This will print `hello, world!`

//     let s = String::from("hello");  // s comes into scope

//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here

//     let x = 5;                      // x comes into scope

//     makes_copy(x);                  // x would move into the function,
//                                     // but i32 is Copy, so it's okay to still
//                                     // use x afterward

// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.


// fn main() {
//     let s1 = gives_ownership();         // gives_ownership moves its return
//                                         // value into s1

//     let s2 = String::from("hello");     // s2 comes into scope

//     let s3 = takes_and_gives_back(s2);  // s2 is moved into
//                                         // takes_and_gives_back, which also
//                                         // moves its return value into s3
// } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
//   // happens. s1 goes out of scope and is dropped.

// fn gives_ownership() -> String {             // gives_ownership will move its
//                                              // return value into the function
//                                              // that calls it

//     let some_string = String::from("yours"); // some_string comes into scope

//     some_string                              // some_string is returned and
//                                              // moves out to the calling
//                                              // function
// }

// // This function takes a String and returns one
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//                                                       // scope

//     a_string  // a_string is returned and moves out to the calling function
// }


// fn main() {
//     let s = String::from("hello world");

//     let hello = &s[0..5];
//     let world = &s[6..11];

//     let slice = &s[0..len];
//     let slice = &s[..];
// }

// fn main() {
//     let my_string = String::from("hello world");

//     // `first_word` works on slices of `String`s, whether partial or whole
//     let word = first_word(&my_string[0..6]);
//     let word = first_word(&my_string[..]);
//     // `first_word` also works on references to `String`s, which are equivalent
//     // to whole slices of `String`s
//     let word = first_word(&my_string);

//     let my_string_literal = "hello world";

//     // `first_word` works on slices of string literals, whether partial or whole
//     let word = first_word(&my_string_literal[0..6]);
//     let word = first_word(&my_string_literal[..]);

//     // Because string literals *are* string slices already,
//     // this works too, without the slice syntax!
//     let word = first_word(my_string_literal);
// }