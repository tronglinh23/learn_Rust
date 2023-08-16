fn main() {
    // condition
    /*
    &&: and
    ||: or
    !: not
    */

    let food = "pizza";
    if food == "cookie" {
        println!("I like cookies");
    } else if food == "pizza" {
        println!("I like pizza");
    } else {
        println!("I don't like {}", food);
    }
}
