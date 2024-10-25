// Data types

let x = 5;  // immutable
let mut y = 10; //mutable
y = 15;

const MAX_POINTS: u32 = 100_000;

let a: i32 = 42;
let b: f64 = 3.14;
let c: bool = true;
let d: char = 'R';

let tuple: (i32, f64, u8) = (500, 6.4, 1);
let arr: [i32; 3] = [1, 2, 3];

let mut s = String::from("hello");
a.push_str(", world!");

let slice:&str = &s[0..5];  // This is immutable - numbers

// Control Flow

let x = 10;
if x > 5 {
    println!("x is greater than 5");
} else {
    println!("x is less than or equal to 5");
}

let y = if x > 5 { 10 } else { 0 };

// Loops

loop {
    println!("Infinite loop");
    break; // Stops the loop
}

let mut n = 3;
while n > 0 {
    println!("{}", n);
    n -= 1;
}

for i in 1..4 {
    println!("{}", i);
}

// switch

let number = 3;

match number { 
    1 => println!("One"),
    2 => println!("Two"),
    3 => println!("Three"),
    _ => println!("Other")   // wildcard
}

// Functions

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn sqaure(x: i32)  -> i32 {
    x * x // No semicolon, returning this value
}

let result1 = add(5, 3);
let result2 = sqaure(5);
println!("Result: {}", result1);  // 8
println("Result: {}", result2)      // 25

// Strctures and enums

struct User {
    username: String,
    email: String,
    active: bool,
}

let user1 = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    active: true,
}

// Instances of struct user


enum IpAddr {
    V4(u8, u8, u8, u8),     // four 8-bit integer
    V6(String), // String type
    Unknown, // Not sata
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
let unknown = IpAddr::Unknown; // not associated data

// Ownership and semantics
// Each value in rust has a variable that's called its owner
// Every value in rust has only one single owner
// When the owner goes out of scope, the value will be dropped

let s3 = String::from("block string");
{
    let s4 = s3;
    println!("Inside block: {}", s4);
}
// s4 is dropped here, memory is freed

println!("Out value: {}", s3) // Will give error, s3 no longet valid

// Referencing and borrowing
fn calculate_length(s: &String) -> usize {
    s.len()
}

let s1 = String::from("hello");
let len = calculate_length(s1);

fn change(s: &mut String) {
    s.push_str(", world");
}

let mut s = String::from("hello");
change(&mut s);

// refernce doesn'transfer ownership, but can be mutable and immutable

// Traits

trait Summary {
    fn summarize(&self) -> String;
}

struct Artcile {
    headline: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}...", &self.content[0..50])
    }
}

