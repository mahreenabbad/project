// use std::io;
// //Countdown Rust

// //input --- 10
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
// #[derive(Debug)]
// struct Pair<T, U> {
//     value1: T,
//     value2: U,
// }

// impl<T, U> Pair<T, U> {
//     // fn value1(&self) -> &T {
//     //     &self.value1
//     // }
//     // fn value2(&self) -> &U {
//     //     &self.value2
//     // }
//     fn swap(self) -> Pair<U, T> {
//         Pair {
//             value1: self.value2,
//             value2: self.value1,
//         }
//     }
// }

// fn main() {
//     let pair = Pair {
//         value1: "Hello",
//         value2: 42,
//     };
//     println!("value1: {:?}", pair.value1);
//     println!("value2: {:?}", pair.value2);
//     let swapped_pair = pair.swap();
//     println!("swapped_pair: {:?}", swapped_pair);

//     // println!(
//     //     "After swap value1 :{}, value2: {}",
//     //     swapped_pair.value1, swapped_pair.value2
//     // );
// }
/////////////////////////////////////////////////
// fn main() {
//     let arr = [
//         String::from("hello"),
//         String::from("world"),
//         String::from("coders"),
//     ];
//     for element in arr.iter() {
//         println!("{:?}", element);
//     }
//     println!("{:?}", arr);
//     for item in arr {
//         println!("{:?}", item);
//     }
// }
/////////////////////////////////////

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
//     fn return_book(&mut self) {
//      6   self.is_available = true;
//     }
// }

// impl Library {
//     fn add_book(&mut self, book: Book) -> Result<(), LibraryError> {
//         if self.book.is_none() {
//             self.book = Some(book);
//             Ok(())
//         } else {
//             Err(LibraryError::BookAlreadyExist)
//         }
//     }
//     fn borrorw_book(&mut self) -> Result<&mut Book, LibraryError> {
//         if let Some(book) = self.book.as_mut() {
//             // using destructring here
//             match book.borrow() {
//                 Ok(borrowed_book) => Ok(borrowed_book),
//                 Err(err) => Err(err),
//             }
//         } else {
//             Err(LibraryError::BookNotFound)
//         }
//     }
//     fn return_book(&mut self) -> Result<(), LibraryError> {
//         if let Some(book) = self.book.as_mut() {
//             book.return_book();
//             Ok(())
//         } else {
//             Err(LibraryError::BookNotFound)
//         }
//     }
// }
// #[derive(Debug)]
// enum LibraryError {
//     BookNotAvailable,
//     BookNotFound,
//     AlreadyBorrowed,
//     BookAlreadyExist,
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
//         book: None, //assuming there is only one book in library of Rust book
//     };
//     match library.add_book(book) {
//         Ok(_) => {
//             println!("Book added to library");
//         }
//         Err(err) => {
//             println!("Error: {:?}", err);
//         }
//     }
//     // attemp to borrow the book and handle result
//     match library.borrorw_book() {
//         Ok(result) => {
//             println!("Borrowed book: {:?}", result);
//         }
//         Err(err) => {
//             println!("Error: {:?}", err);
//         }
//     }
//     match library.return_book() {
//         Ok(_) => {
//             println!("Returned book sucessfully:");
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
/////////////////////////////////////////////////
/// trait
// struct Student {
//     name: String,
// }
// trait Name {
//     fn change_name(&mut self, new_name: String);
// }
// impl Name for Student {
//     fn change_name(&mut self, new_name: String) {
//         self.name = new_name;
//     }
// }
// fn main() {
//     let mut student = Student {
//         name: String::from("Mahreen"),
//     };
//     println!("Before changing name: {}", student.name);
//     student.change_name(String::from("Abbad"));
//     println!("After changing name: {}", student.name);
// }
//////////////////
//generic types

// pub fn num<T: std::fmt::Display>(value: T) {
//     println!("The number is: {}", value);
// }
// fn main() {
//     let value = 10;

//     num(value);
//     let status = true;
//     num(status);
//     let number = 5.5;
//     num(number);
// }
///////////////////////////////////
///
// struct Container<T> {
//     value: T,
// }

// impl<T: Clone> Container<T> {
//     fn new(new_value: T) -> Self {
//         Self { value: new_value }
//     }
//     fn set(&mut self, new_value: T) {
//         self.value = new_value;
//     }

//     fn get(&self) -> T {
//         self.value.clone()
//     }
// }
// fn main() {
//     let mut container = Container::new(5);
//     println!("Value: {}", container.get());
//     container.set(10);
//     println!("Value: {}", container.get());

//     let mut container = Container::new(true);
//     println!("Value: {}", container.get());
//     container.set(false);
//     println!("Value: {}", container.get());

//     let mut container = Container::new(String::from("Raj"));
//     println!("Value: {}", container.get());
//     container.set(String::from("Verma"));
//     println!("Value: {}", container.get());
// }
/////////////////////////////////////////////
/// write a function that remove all white spaces from a string
// use std::io;
// fn main() {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Invalid Input");
//     remove_whitespace(&mut input);
// }
// fn remove_whitespace(input: &mut String) {
//     //
//     // let words = input.split_whitespace().collect::<Vec<&str>>().join("");
//     // println!("{:?}", words);
//     // let words = input.split_whitespace();

//     // let vec: Vec<&str> = words.collect();
//     // println!("{:?}", vec);
//     // let Joined_word = vec.join("");
//     // println!("{:?}", Joined_word);
//     let trimmed_input = input.trim();
//     let character = trimmed_input.chars();
//     let reverse_char = character.rev();
//     let reverse_string: String = reverse_char.collect();
//     println!("{:?}", reverse_string);
// }
////////
/// palindrome
///
// fn palindrome_check(input: String) {
//     let original_str: String = input.clone();
//     let reverse_string: String = input.trim().chars().rev().collect();
//     println!("{}", original_str == reverse_string);
// }

// fn main() {
//     palindrome_check("wow".to_string());
// }
//Counter
////////////////
// struct Counter {
//     counter: u32,
// }
// impl Counter {
//     fn new() -> Self {
//         Counter { counter: 0 }
//     }
// }
// impl Iterator for Counter {
//     type Item = u32;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.counter = self.counter + 1;

//         if self.counter < 5 {
//             Some(self.counter)
//         } else {
//             None
//         }
//     }
// }
// fn main() {
//     let mut counter = Counter::new();
//     while let Some(value) = counter.next() {
//         println!("{}", value);
//     }
// }
////////////////////////////
///
//CLOSURE
// fn main() {
// let mut counter = 0;
// let mut increament = || {
//     counter += 1;
//     println!("Counter: {}", counter);
// };
// increament();
// increament();
// increament();
//     let x = 10;
//     let add_number = |y: i32| y + x;
//     println!("{}", add_number(20));
//     println!("{}", add_number(30));
// }

// fn main() {
//     let values = vec![10, 20, 30, 79, 31, 40, 50];
//     let even_vector: Vec<i32> = values.into_iter().filter(|x| x % 2 == 0).collect();
//     println!("{:?}", even_vector);
// }
// how to apply ownership on closure

// fn main() {
//     let x = String::from("hello world");
//     let consume_and_return_x = || &x;
//     println!("{}", x);
//     let y = consume_and_return_x();
//     println!("{}", y);
//     let z = consume_and_return_x();
//     println!("{}", z);
//     let mut x_mut = x.clone();

// }
/////////////////////
/// TO DO list -addtask - remove task, exit
use std::io::{self, Read};
#[derive(Debug)]
struct Task {
    description: String,
    priority: u8,
    completed: bool,
}
impl Task {
    fn new(description: String, priority: u8) -> Self {
        Self {
            description: description,
            priority: priority,
            completed: false,
        }
    }
}
fn main() {
    let mut task_list: Vec<Task> = Vec::new();
    loop {
        let mut choice = String::new();
        println!("Please enter your choice");
        println!("1- Add Task");
        println!("2- Remove Task");
        println!("3- View Task");
        println!("4- view completed task");
        println!("5- view pending task");
        println!("6- Mark complete");
        println!("7- change priorty");
        // choice.clear();
        io::stdin().read_line(&mut choice).expect("Invalid Input");
        let choice: i32 = choice.trim().parse().expect("Invalid number");

        match choice {
            1 => add_task(&mut task_list),
            2 => remove_task(&mut task_list),
            3 => view_task(&task_list),
            4 => view_completed_task(&task_list),
            5 => view_pending_task(&task_list),
            6 => mark_completed(&mut task_list),
            7 => change_priorty(&mut task_list),
            8 => {
                println!("Exiting the program");
                break;
            }
            _ => println!("Invalid Choice"),
        }
    }
}

fn view_completed_task(task_list: &Vec<Task>) {}
fn view_pending_task(task_list: &Vec<Task>) {}
fn mark_completed(task_list: &mut Vec<Task>) {}
fn change_priorty(task_list: &mut Vec<Task>) {}
fn add_task(task_list: &mut Vec<Task>) {
    let mut description = String::new();
    println!("Please enter the task description:");
    io::stdin()
        .read_line(&mut description)
        .expect("Invalid Input");
    let description = description.trim().to_string();

    let mut priority = String::new();
    println!("Please enter the task priority:");
    io::stdin().read_line(&mut priority).expect("Invalid Input");
    let priority: u8 = priority.trim().parse().expect("Invalid number");

    if !description.is_empty() && (1..=5).contains(&priority) {
        task_list.push(Task::new(description, priority));
    } else {
        println!("Description or priority is invalid");
    }
}
fn remove_task(task_list: &mut Vec<Task>) {
    if task_list.is_empty() {
        println!("No task to remove");
        return;
    }
    println!("Enter task number to remove task");

    view_task(task_list);
    let mut task_number = String::new();
    io::stdin()
        .read_line(&mut task_number)
        .expect("Invalid Input");
    match task_number.trim().parse::<usize>() {
        Ok(task_number) => {
            if task_number > task_list.len() {
                println!("wrong task number");
                return;
            }
            task_list.remove(task_number - 1);
            println!("Task removed successfully");
        }
        Err(_) => {
            println!("error situation");
            return;
        }
    }
}
//view task function
fn view_task(task_list: &Vec<Task>) {
    if task_list.is_empty() {
        println!("No task to show");
        return;
    }
    println!("Task List:{:?} ", task_list);
}

//edit task function

// fn edit_task(task_list: &mut Vec<Task>) {
//     if task_list.is_empty() {
//         println!("No task to edit");
//     }
//     println!("Enter task number to' edit task");
//     view_task(task_list);
//     let mut task_number = String::new();
//     io::stdin()
//         .read_line(&mut task_number)
//         .expect("Invalid Input");
//     let task_number: usize = task_number.trim().parse().expect("Invalid number");
//     println!("type update task");
//     let mut new_task = String::new();
//     io::stdin()
//         .read_line(&mut new_task)
//         .expect("something wentwrong");
//     task_list[task_number - 1] = new_task.trim().to_string();
// }
