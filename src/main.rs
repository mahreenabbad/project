// //Countdown Rust
// //input - 10
// //10 9 8 7 6 5 ...1
// use std::io;
// use std::thread::sleep;
// use std::time::Duration;

// fn main() {
//     loop {
//         let mut input = String::new();
//         println!("Please enter the timer:");
//         io::stdin().read_line(&mut input).expect("Invalid Input");

//         // let timer: u16 = input.trim().parse().expect("Invalid number");//If you need the input to be of a specific type
//         let timer: u16 = match input.trim().parse() {
//             Ok(timer) => timer,
//             Err(_) => {
//                 println!("Invalid number");
//                 continue;
//             }
//         };
//         start_timer(timer);
//         break;
//     }
// }

// //1 2 3 4 5 6 .. 10 -> (1..=10)
// //10 9 8 7 6 5 ... 1 -> (1..=10).rev()
// fn start_timer(timer: u16) {
//     for i in (1..=timer).rev() {
//         println!("Timer countdown...{}", i);
//         sleep(Duration::from_secs(1)); //to have a delay of 1 second
//     }
// }
///////////////////////////////////////////////

// convert natural number to binary
// use std::io;
// fn main() {
//     let mut input = String::new();
//     println!("Please enter the decimal number:");
//     io::stdin().read_line(&mut input).expect("invalid input");
//     let mut decimal_value: u32 = match input.trim().parse() {
//         Ok(decimal_value) => decimal_value,
//         Err(_) => {
//             println!("Invalid number");
//             return;
//         }
//     };
//     binary_converter(decimal_value);
//     hex_converter(decimal_value);
// }
// fn binary_converter(mut decimal_value: u32) {
//     let mut binary = String::new();
//     while decimal_value > 0 {
//         let remainder: u8 = (decimal_value % 2) as u8;
//         binary.insert(0, (remainder + 48) as char);
//         // print!("{}", remainder);
//         decimal_value = decimal_value / 2;
//     }
//     println!("Binary number :{}", binary);
//     print!("\n");
// }
// fn hex_converter(mut decimal_value: u32) {
//     let mut hex = String::new();
//     while decimal_value > 0 {
//         let remainder = (decimal_value % 16) as u8;
//         if remainder > 9 {
//             hex.insert(0, (remainder + 55) as char);
//             // print!("{}", (remainder + 55) as char);
//         } else {
//             hex.insert(0, (remainder + 48) as char);
//             // print!("{}", remainder);
//         }
//         decimal_value = decimal_value / 16;
//     }
//     println!("Hexadecimal number :{}", hex);
// }
//////////////////////////////////////////////////
/// //  create a baking application
// take input from the user
// 1- deposit money
//2- withdraw money
//3-check user balance
//4- exit
// if user make any wrong choice you have to ask user to retry

// use std::io;
// fn main() {
//     let mut user_balance: f64 = 0.0;
//     let mut flag: bool = false;
//     loop {
//         let mut input = String::new();
//         println!("please enter your choice");
//         println!("1- deposit money");
//         println!("2- withdraw money");
//         println!("3- fetch balance");
//         println!("4- exit");
//         io::stdin().read_line(&mut input).expect("Invalid Input");
//         let choice: u8 = match input.trim().parse() {
//             Ok(choice) => choice,
//             Err(_) => {
//                 println!("Invalid Number");
//                 continue;
//             }
//         };
//         execute(choice, &mut user_balance, &mut flag);
//         //println!("User current Balance : {}", user_balance);
//         if flag == true {
//             break;
//         }
//     }
// }
// fn execute(choice: u8, user_balance: &mut f64, flag: &mut bool) {
//     match choice {
//         1 => deposit(user_balance),
//         2 => withdraw(user_balance),
//         3 => fetch_balance(user_balance),
//         4 => exit(flag),
//         _ => println!("Invalid Choice"),
//     }
// }
// fn deposit(user_balance: &mut f64) {
//     let mut input = String::new();
//     println!("Please enter the amount to deposit:");
//     io::stdin().read_line(&mut input).expect("Invalid Input");
//     let amount: f64 = match input.trim().parse() {
//         Ok(choice) => choice,
//         Err(_) => {
//             println!("Invalid Number");
//             return;
//         }
//     };
//     *user_balance = *user_balance + amount;
// }

// fn withdraw(user_balance: &mut f64) {
//     let mut input = String::new();
//     println!("Please enter your amount");
//     io::stdin().read_line(&mut input).expect("Invalid Input");

//     let amount: f64 = match input.trim().parse() {
//         Ok(choice) => choice,
//         Err(_) => {
//             println!("Invalid Number");
//             return;
//         }
//     };
//     if *user_balance < amount {
//         return;
//     }
//     *user_balance = *user_balance - amount;
// }

// fn fetch_balance(user_balance: &mut f64) {
//     println!("User current Balance : {}", user_balance);
// }
// fn exit(flag: &mut bool) {
//     *flag = true;
// }
//////////////////////////////////////////////////
///
///
// #[derive(Debug)]
// struct Student {
//     name: String,
//     age: u8,
//     grade: f32,
// }
// fn main() {
//     let student1: Student = Student {
//         name: String::from("John"),
//         age: 20,
//         grade: 60.5,
//     };
//     let student2 = &student1;
//     // let student2 = Student {
//     //     name: String::from("Sina"),
//     //     ..student1
//     // };
//     println!("student details : {:#?}", student1);
//     println!("student details : {:#?}", student2);
// }
//////////////////////////////////////////
// #[derive(Debug)] // to print struct = {:?}
// struct Rectangle {
//     length: u16,
//     breadth: u16,
// }

// impl Rectangle {
//     fn area(&self) -> u16 {
//         self.length * self.breadth
//     }
//     fn update_len(&mut self) {
//         self.length = 50
//     }
// }
// fn area_rec(rec: &Rectangle) -> u16 {
//     rec.length * rec.breadth
// }
// fn update_len(rec: &mut Rectangle) {
//     rec.length = 5;
// }

// fn main() {
//     let mut rec = Rectangle {
//         length: 10,
//         breadth: 20,
//     };
//     let result = area_rec(&rec);
//     println!("Area of Rec :{:#?}", result);

//     update_len(&mut rec);
//     let result1 = area_rec(&rec);
//     println!("Area of Rec :{:#?}", result1);

//     println!("Area of Rec via Method : {}", rec.area());
//     println!("Area of Rec via Method : {}", rec.area());
//     rec.update_len();
//     println!("Area of Rec via Method : {}", rec.area());
// }
////////////////////////////////////
#[derive(Debug)]
struct Pair<T, U> {
    value1: T,
    value2: U,
}

impl<T, U> Pair<T, U> {
    fn value1(&self) -> &T {
        &self.value1
    }
    fn value2(&self) -> &U {
        &self.value2
    }
    fn swap(self) -> Pair<U, T> {
        Pair {
            value1: self.value2,
            value2: self.value1,
        }
    }
}

fn main() {
    let pair = Pair {
        value1: "Hello",
        value2: 42,
    };
    println!("value1: {:?}", pair.value1);
    println!("value2: {:?}", pair.value2);
    let swapped_pair = pair.swap();
    println!("swapped_pair: {:?}", swapped_pair);
    // println!(
    //     "After swap value1 :{}, value2: {}",
    //     swapped_pair.value1, swapped_pair.value2
    // );
}
/////////////////////////////
// #[derive(Debug)]
// struct Book {
//     title: String,
//     author: String,
//     is_available: bool,
// }
// #[derive(Debug)]
// struct Library {
//     name: String,
//     address: String,
//     book: Option<Book>,
// }

// impl Book {
//     fn borrow(&mut self) -> Result<&mut Book, LibraryError> {
//         // used result enum here
//         if self.is_available == true {
//             self.is_available = false;
//             Ok(self)
//         } else {
//             Err(LibraryError::AlreadyBorrowed)
//         }
//     }
//     fn return_book() {}
// }

// impl Library {
//     fn add_book() {}
//     fn borrorw_book(&mut self) -> Result<&mut Book, LibraryError> {
//         if let Some(book) = self.book.as_mut() {
//             // using destructring here
//             match book.borrow() {
//                 Ok(borrowed_book) => {
//                     println!("Book is borrowed");
//                     Ok(borrowed_book)
//                 }
//                 Err(err) => {
//                     println!("Book is already borrowed");
//                     Err(err)
//                 }
//             }
//         } else {
//             Err(LibraryError::BookNotFound)
//         }
//     }
//     fn return_book() {}
// }
// #[derive(Debug)]
// enum LibraryError {
//     BookNotAvailable,
//     BookNotFound,
//     AlreadyBorrowed,
// }
// fn main() {
//     let book = Book {
//         title: String::from("The Rust Book"),
//         author: String::from("Steve Klabnik"),
//         is_available: true,
//     };
//     let mut library = Library {
//         name: String::from("City Library"),
//         address: String::from("123 Main St"),
//         book: Some(book), //assuming there is only one book in library of Rust book
//     };
//     // attemp to borrow the book and handle result
//     match library.borrorw_book() {
//         Ok(result) => {
//             println!("Borrowed book: {:?}", result);
//         }
//         Err(err) => {
//             println!("Error: {:?}", err);
//         }
//     }
//     // try borrow it again
//     match library.borrorw_book() {
//         Ok(result) => {
//             println!("Borrowed book: {:?}", result);
//         }
//         Err(err) => {
//             println!("Error: {:?}", err);
//         }
//     }
// }
