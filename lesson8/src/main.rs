fn main() {
    test_one();
    add_number(10, 20);

    // expression, not statement

    let number = {
        let x = 3;
        x + 1
    };

    println!("number = {}", number);

    let result = add_number2(10, 20);
    println!("result = {}", result);
}

fn test_one() {
    println!("test_one");
}

// return statement
fn add_number(x: i32, y: i32) {
    println!("{} + {} = {}",x,y,x + y);
}

// return expression
fn add_number2(x: i32, y: i32) -> i32 {
    x + y

    // or use this syntax
    // return x + y;
}

