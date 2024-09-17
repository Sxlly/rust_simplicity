// Shae Sullivan
// 20643731
// Programming Languages - Assignment
// 17/09/24



fn main() {

    let x = 4; //immutable variables by default
    println!("x is: {}", x); // by default cannot change "x" without redeclaring the variable

    // to make "x" be able to be changes without being redefinied in rust you have to declare it as mutable in the variables declaration

    let mut x = 5; //this is very complex and not simple in comparison to a language like python for example
    println!("x is: {}", x);

    //although the following code is valid in rust where as in python it would be "x is already definied"
    let y = 6;
    println!("y is: {}", y);
    let y = 8; //redefining is fine in rust as thats what it wants the user to do! as y as a variable is unmutable by default
    println!("y is: {}", y);

    //another simplicity hinderance in rust is when defining constants for example
    const SECONDS_IN_MINUTE: u32 = 60; // "u32"? is unsigned interger 32 which is the type of this constant, you must define this in rust in opposed to python for example
    println!("{}", SECONDS_IN_MINUTE); //you also cant redfine this constant in rust like you can in python

}
