// fn main() {
//     const DECIMAL:u32 = 100;
//     let mut x = 5 * DECIMAL;
//     print!("The value of x is: {x}");

//     x= 6 * DECIMAL;
//     print!("The value of x is {x}");
// }
// // Constants:
// // constants are values bound to a name and are not allowed to change,  but there are a few difference between constants and variables.

// // You are not allowed to use the mut key word constants
// // Data types:
// /*
// scaler and compounde types

// A scalar represent a single value. Rust has four primary scalar types: integers, booleans, numbers, characters, floating-points.
// Integers : u8, u16 .... usize ^ i8, i16 .... isize.
// Each variants can store numbers from -(2^n-1) to 2^n-1

// Character vs string literal
/*
char are represented in single quotes e.g let c = 'C'; while string literals are represented with doube=le quotes e.g let name = "John"
*/

// floating number: number with decimal point e.g 2.0 e.g  x:f32 = 2.0

// compound type
/*
Tuples and Arrays:

let a: (f32, u32, u8) = (2.0, 65, 1)

The tuple without any values has a special name, unit. This value and its corresponding type are both written () and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.



Array Type:
contains element of same data types and are not growable i.e of fixed length.
Array are useful when you want your data located on stack instead of heap (check out more on stacks and heaps)
arrays are useful when you know the number of elements will not change.
*/

// fn main (){
//     // let tup = (500, 6.4, 1);
//     // let (x,y,z) = tup;
//     // print!("Result of tuples x is: {x}");

//     // print!("Result of tuples y is: {y}");

//     // print!("Result of tuples z is: {z}")
//     let months = ["January", "February", "March", "April", "May", "June", "July",
//         "August", "September", "October", "November", "December"];

//      //fig1 let variable: [data_type; num_of_element]
//     //  using fig1, let a:[i32; 5] = [1,2,3,4,5];
//     // You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and the length of the array in square bracket
//     // let a = [3;5];
//     // The array named a will contain 5 elements that will all be set to the value 3 initially. This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.

// // for  i in months{
// //     println!("output: {i}");
// // }

// let mut index = 8;

// loop{
//     println!("{}", months[index]);

//     index +=1;

//     if index >= months.len(){
//         break;
//     }
// }

// }

// use std::io;
// fn main() {
//     let mut names: Vec<String> = Vec::new();

//     println!("please enter name.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let name = index.trim().to_string();
//     names.push(name);

//     println!("The name was pushed in to the array {:?}:", names);
//     for i in &names{
//         println!("name: {i}");
//     }
//     // {:?} uses the Debug trait formatting, which is typically more detailed and includes additional information for debugging.
// // {} uses the Display trait formatting, which is usually more user-friendly and intended for regular display to users.
// }

/*
FUNCTIONS:
Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value. Let’s look at some examples.

*/

/*
This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.

Two tables: the first table contains the representation of s1 on the
stack, consisting of its length (5), capacity (5), and a pointer to the first
value in the second table. The second table contains the representation of the
string data on the heap, byte by byte.

Figure 4-1: Representation in memory of a String holding the value "hello" bound to s1

The length is how much memory, in bytes, the contents of the String are currently using. The capacity is the total amount of memory, in bytes, that the String has received from the allocator. The difference between length and capacity matters, but not in this context, so for now, it’s fine to ignore the capacity.

*/

/*

All the integer types, such as u32.
The Boolean type, bool, with values true and false.
All the floating-point types, such as f64.
The character type, char.
Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
*/

// fn main() {
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

// 11|9|2023 - Understanding Ownership and Using Structs to Structure related data:

/*
References are immutable by defaults just like variable, you have to explicitly define the `&mut` to mutate references.

Restriction of mutable references: if you have mutable ref to a vaue, you can have no other reference to that value, i.e you can only have one mutable ref for a value.


*/

// fn main() {
//   let mut s = String::from("hello");

//   change(&mut s);
//   change_2(&mut s);
//   println!("{}", s.trim())
// }

// fn change(some_string: &mut String) {
//   some_string.push_str(", world");
// }

// fn change_2(some_string: &mut String) {
//   some_string.push_str(", comrade");
// }

// mod aggregator;
// fn main() {
//     const DECIMAL:u32 = 100;
//     let mut x = 5 * DECIMAL;
//     print!("The value of x is: {x}");

//     x= 6 * DECIMAL;
//     print!("The value of x is {x}");
// }
// // Constants:
// // constants are values bound to a name and are not allowed to change,  but there are a few difference between constants and variables.

// // You are not allowed to use the mut key word constants
// // Data types:
// /*
// scaler and compounde types

// A scalar represent a single value. Rust has four primary scalar types: integers, booleans, numbers, characters, floating-points.
// Integers : u8, u16 .... usize ^ i8, i16 .... isize.
// Each variants can store numbers from -(2^n-1) to 2^n-1

// Character vs string literal
/*
char are represented in single quotes e.g let c = 'C'; while string literals are represented with doube=le quotes e.g let name = "John"
*/

// floating number: number with decimal point e.g 2.0 e.g  x:f32 = 2.0

// compound type
/*
Tuples and Arrays:

let a: (f32, u32, u8) = (2.0, 65, 1)

The tuple without any values has a special name, unit. This value and its corresponding type are both written () and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.



Array Type:
contains element of same data types and are not growable i.e of fixed length.
Array are useful when you want your data located on stack instead of heap (check out more on stacks and heaps)
arrays are useful when you know the number of elements will not change.
*/

// fn main (){
//     // let tup = (500, 6.4, 1);
//     // let (x,y,z) = tup;
//     // print!("Result of tuples x is: {x}");

//     // print!("Result of tuples y is: {y}");

//     // print!("Result of tuples z is: {z}")
//     let months = ["January", "February", "March", "April", "May", "June", "July",
//         "August", "September", "October", "November", "December"];

//      //fig1 let variable: [data_type; num_of_element]
//     //  using fig1, let a:[i32; 5] = [1,2,3,4,5];
//     // You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and the length of the array in square bracket
//     // let a = [3;5];
//     // The array named a will contain 5 elements that will all be set to the value 3 initially. This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.

// // for  i in months{
// //     println!("output: {i}");
// // }

// let mut index = 8;

// loop{
//     println!("{}", months[index]);

//     index +=1;

//     if index >= months.len(){
//         break;
//     }
// }

// }

// use std::io;
// fn main() {
//     let mut names: Vec<String> = Vec::new();

//     println!("please enter name.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let name = index.trim().to_string();
//     names.push(name);

//     println!("The name was pushed in to the array {:?}:", names);
//     for i in &names{
//         println!("name: {i}");
//     }
//     // {:?} uses the Debug trait formatting, which is typically more detailed and includes additional information for debugging.
// // {} uses the Display trait formatting, which is usually more user-friendly and intended for regular display to users.
// }

/*
FUNCTIONS:
Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value. Let’s look at some examples.

*/

/*
This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.

Two tables: the first table contains the representation of s1 on the
stack, consisting of its length (5), capacity (5), and a pointer to the first
value in the second table. The second table contains the representation of the
string data on the heap, byte by byte.

Figure 4-1: Representation in memory of a String holding the value "hello" bound to s1

The length is how much memory, in bytes, the contents of the String are currently using. The capacity is the total amount of memory, in bytes, that the String has received from the allocator. The difference between length and capacity matters, but not in this context, so for now, it’s fine to ignore the capacity.

*/

/*

All the integer types, such as u32.
The Boolean type, bool, with values true and false.
All the floating-point types, such as f64.
The character type, char.
Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
*/

// fn main() {
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

// 11|9|2023 - Understanding Ownership and Using Structs to Structure related data:

/*
References are immutable by defaults just like variable, you have to explicitly define the `&mut` to mutate references.

Restriction of mutable references: if you have mutable ref to a vaue, you can have no other reference to that value, i.e you can only have one mutable ref for a value.

A data race is similar to a race condition and happens when these three behaviors occur:

Two or more pointers access the same data at the same time.
At least one of the pointers is being used to write to the data.
There’s no mechanism being used to synchronize access to the data.

---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
Dangling References
In languages with pointers, it’s easy to erroneously create a dangling pointer—a pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

String Slices:



*/

/*
STRUCT
1. struct
2. tuple struct
3. unit-like struct

To store reference in a struct, we use lifetime, if you dont specify lifetime, it won't work.

*/

// struct Rectangle{
//   width:u32,
//   height:u32
// }

// fn main(){
// let rec = Rectangle{
//   width:30,
//   height:50
// };

// let rec2 = Rectangle{
//   width:90,
//   height:90
// };
// let result = area(&rec);
// println!("The area of the rectangle is {} pixels", result);

// let result2 = area(&rec2);
// println!("The area of the rectangle is {} pixels", result2)
// }
// fn area (rectangle: &Rectangle) -> u32{
//  rectangle.width * rectangle.height
// }
// How echo Works: How to use arguement of the command-line to chnage behavior of a program at run-time.

/*
In this chapter, you’ll learn how to do the following:
• Process command-line arguments with the clap crate
• Use Rust types like strings, vectors, slices, and the unit type
• Use expressions like match, if, and return
• Use Option to represent an optional value
• Handle errors using the Result variants of Ok and Err
• Understand the difference between stack and heap memory
• Test for text that is printed to STDOUT and STDERR



*/

use clap::{App, Arg};
fn main() {
    let matches = App::new("echo")
        .version("0.1.0")
        .author("Adegbite Ademola-Kelvin <Ademol@web3bridge.com>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();
    //   println!("{:#?}",matches)
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    print!(
        "{} {}",
        text.join(" "),
        if omit_newline { "" } else { "\n" }
    );
}

