fn main() {
    /* data types */

    let x: i32 = 2;
    println!("x = {}", x);

    /* signed integer
    i8 : -2^7 -> 2^7 - 1
    i16
    i32: default 
    i64
    i128
    */

    /* unsigned integer
    u8 : 0 -> 2^8
    u16
    u32
    u64
    u128
    */

    /* float
    f32 f64
    */
    let f: f32 = 2.0;
    println!("f = {}", f);

    /* boolean 
    false
    true
    */
    let b: bool = true;

    /* character
    */

    let letter: char = 'a';

    /* compound types */
    /* tuple */

    let mut tup: (i32, bool, char) = (1, true, 's');
    tup.0 = 2;
    println!("tup[0] = {}", tup.0);
    println!("tup = {:?}", tup);

    // type: [type;size]
    let mut arr: [i32;5] = [1,2,3,4,5];
    
    arr[4] = 3;
    println!("arr = {:?}", arr);

}
