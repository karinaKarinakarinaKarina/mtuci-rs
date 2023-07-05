// // chapter five

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };
// }



// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };
//     dbg!(&rect1);
// }


// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }



// // chapter sixth



// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }
// let home = IpAddr::V4(127, 0, 0, 1);
// let loopback = IpAddr::V6(String::from("::1"));



// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }


// let mut count = 0;
// if let Coin::Quarter(state) = coin {
//     println!("State quarter from {:?}!", state);
//   } else {
//     count += 1;
// }




// // chapter 8



// let mut v = Vec::new();
// v.push(5);
// v.push(6);
// v.push(7);



// fn main() {
//     let v = vec![1, 2, 3, 4, 5];
//     let third: &i32 = &v[2];
//     println!("The third element is {third}");
//     let third: Option<&i32> = v.get(2);
//     match third {
//         Some(third) => println!("The third element is {third}"),
//         None => println!("There is no third element."),
//     }
// }




// fn main() {
//     enum SpreadsheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }

//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.12),
//     ];
// }


// fn main() {
//     use std::collections::HashMap;
//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
//     let team_name = String::from("Blue");
//     let score = scores.get(&team_name).copied().unwrap_or(0);
// }



// fn main() {
//     use std::collections::HashMap;

//     let text = "hello world wonderful world";

//     let mut map = HashMap::new();

//     for word in text.split_whitespace() {
//         let count = map.entry(word).or_insert(0);
//         *count += 1;
//     }

//     println!("{:?}", map);
// }


