fn main() {

    //// first 

    /* implicit type */

    // let mut x = 4;
    // println!("The value of x is: {}", x);

    /* error : cannot assign twice to immutable variable `x`, immutable by default */

    // x = 5;
    // println!("The value of x is: {}", x);

    /* 
    to fix this we can use mutable variable
    let mut x = 4
    */



    //// second
    // you can recreate new variable with same name

    let x = 5;
    println!("The value of x is: {}", x);
    let x = "hello";
    println!("The value of x is: {}", x);

    /* const variable*/
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("The value of SECONDS_IN_MINUTE is: {}", SECONDS_IN_MINUTE);

}
