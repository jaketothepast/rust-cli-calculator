use std::env;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide
}


// Functions must denote type as part of args.
// Functions must denote return type after -> after args!
fn sum(a:i32, b:i32) -> i32 {
    // No semicolon == return type
    a + b
}

fn multiply(a:i32, b:i32) -> i32 {
    a * b
}

fn subtract(a:i32, b:i32) -> i32 {
    a - b
}

fn divide(a:i32, b:i32) -> i32 {
    a / b
}


// Because we could possibly return None, must use option
fn do_operation(op:&str, a: i32, b: i32) -> Option<i32> {
    match op {
        "+" => Some(sum(a, b)),
        "-" => Some(subtract(a,b)),
        "*" => Some(multiply(a,b)),
        "/" => Some(divide(a,b)),
        _ => None
    }
}

// Must get reference to str because String has ownership of data.
fn get_arg(arg: Option<&String>) -> &str{
    // Use match to unroll our arg
    match arg {
        Some(inner) => &inner,
        None => "nothing",
    }
}

fn main() {

    // Collect the arguments into a Vector of strings.
    let args: Vec<String> = env::args().collect();

    // args.get returns an option!
    let op: &str = get_arg(args.get(1));
    // Call parse on typed variables, hopefully get int!
    // Call unwrap so that I do not have to explicitly unroll the results.
    let val1: i32 = get_arg(args.get(2)).parse().unwrap();
    let val2: i32 = get_arg(args.get(3)).parse().unwrap();

    let mut avec = Vec::new();

    // Can push to the vector
    avec.push(2.2);
    avec.push(9.2);
    avec.push(2.);

    // Can pop from the vector
    avec.pop();

    // Can assign to the vector
    avec[0] = 3.9;

    println!("Avec value? {:?}", avec);

    for i in &avec {
       println!("Our reference to avec, non-mutable {}", i);
    }

    for i in avec {
        println!("We are taking ownership of avec {}", i);
    }

    // This will never work due to above loop taking ownership of avec
    // We moved avec.
    println!("Avec value? {:?}", avec);

    println!("First arg! {}", op);
    println!("Val 1! {}", val1);
    println!("Val 2! {}", val2);

    match do_operation(op, val1, val2) {
        Some(res) => println!("Result was: {}", res),
        None => println!("Whoops, try again!")
    }
}
