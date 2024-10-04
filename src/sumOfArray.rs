// Shae Sullivan
// 20643731
// Programming Languages - Assignment
// 17/09/24


fn main() { //declare main funciton 

    let mut array: [i32; 5] = [10,20,30,40,50]; //declare array with all contents having a 32 bit signed integer data type and 5 index length

    let arraysum: i32 = array.iter().sum(); // decalre arraysum variable as 32 bit signed integer -> use array iter method to iterate through array and sum method to get sum

    println!(

        "sum of array is {}",
        arraysum
    );

}