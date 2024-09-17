// Shae Sullivan
// 20643731
// Programming Languages - Assignment
// 17/09/24



fn main() {

    let x = 4; //immutable variables by default
    println!("x is: {}", x); // by default cannot change "x" without redeclaring the variable

    // to make "x" be able to be changes without being redefinied in rust you have to declare it as mutable in the variables declaration

    let mut x = 5; //this is very complex and not simple in comparison to a language like python for example
    println!("x is {}", x);
}
